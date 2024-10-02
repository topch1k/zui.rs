use std::mem;

use futures::TryFutureExt;
use zookeeper_async::Acl;

use crate::node_data::NodeData;

use super::App;

impl App {
    pub(crate) async fn store_node_stat(&mut self) {
        let full_path = self.full_resource_path();
        let _ = self
            .zk
            .as_ref()
            .unwrap()
            .exists(&full_path, false)
            .and_then(|stat| async {
                self.current_node_stat = stat;
                Ok(())
            })
            .await;
    }

    pub(crate) async fn store_children(&mut self) {
        {
            self.clear_message();
            self.append_message(format!("Full path : {}\n", self.full_resource_path()));
            self.append_message(format!("Prev path : {:?}\n", self.prev_resources));
        }
        let Some(ref zk) = self.zk else {
            return;
        };

        let children = zk
            .get_children(&self.full_resource_path(), false)
            .await
            .ok();
        if let Some(ch) = children {
            self.tab_data = ch;
        }
    }

    pub(crate) async fn store_node_data(&mut self) {
        let Some(ref zk) = self.zk else {
            return;
        };

        let _ = zk
            .get_data(&self.full_resource_path(), false)
            .and_then(|(data, _)| async {
                self.node_data = NodeData::Raw(data);
                Ok(())
            })
            .await;
    }

    pub(crate) async fn create_node(&mut self) {
        let Some(ref zk) = self.zk else {
            "Failed to get zookeeper client".clone_into(&mut self.message);
            return;
        };

        let res = zk
            .create(
                &self.node_path_buf,
                self.node_data_buf.clone().into_bytes(),
                Acl::open_unsafe().clone(),
                zookeeper_async::CreateMode::Persistent,
            )
            .await;
        match res {
            Ok(created_path) => self.message = format!("Node {created_path} created successfully"),
            Err(e) => self.message = format!("Node creation failed : {e}"),
        }
    }

    pub(crate) async fn set_data(&mut self) {
        let Some(ref zk) = self.zk else {
            "Failed to get zookeeper client".clone_into(&mut self.message);
            return;
        };

        let data = mem::take(&mut self.node_data_buf).into_bytes();
        let res = zk.set_data(&self.full_resource_path(), data, None).await;
        match res {
            Ok(_) => {
                self.message = format!(
                    "Node {} data successfully updated",
                    self.full_resource_path()
                )
            }
            Err(e) => self.message = format!("Node data update failed : {e}"),
        }
    }

    pub(crate) async fn delete_node(&mut self) {
        let Some(ref zk) = self.zk else {
            "Failed to get zookeeper client".clone_into(&mut self.message);
            return;
        };
        let res = zk.delete(&self.full_resource_path(), None).await;
        match res {
            Ok(_) => {
                self.message = format!("Node {} successfully deleted", self.full_resource_path())
            }
            Err(e) => self.message = format!("Delete node failed : {e}"),
        }
    }
}
