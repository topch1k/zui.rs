use super::App;
use crate::{
    errors::{AppError, AppResult},
    node_data::NodeData,
    zk::LoggingWatcher,
};
use std::{mem, time::Duration};
use zookeeper_async::{Acl, ZooKeeper};

impl App {
    pub(crate) async fn connect_default(connection_str: &str) -> AppResult<ZooKeeper> {
        //TODO: Make timeout configurable
        //TODO: Design Failure window
        tokio::select! {
            res = ZooKeeper::connect(connection_str, Duration::from_secs(1), LoggingWatcher) => res.map_err(Into::into),
            _ = tokio::time::sleep(Duration::from_secs(3)) => {
                Err(AppError::ConnectionTimeoutError)
            }
        }
    }

    pub(crate) async fn store_node_stat(&mut self) {
        let full_path = self.tab_full_resource_path();
        let _ = self
            .zk
            .as_ref()
            .unwrap()
            .exists(&full_path, false)
            .await
            .map(|stat| {
                self.curr_tab_mut().current_node_stat = stat;
            });
    }

    pub(crate) async fn store_children(&mut self, children: Vec<String>) {
        {
            self.clear_tab_message();
            self.append_tab_message(format!("Full path : {}\n", self.tab_full_resource_path()));
        }

        self.curr_tab_mut().tab_data = children;
    }

    pub(crate) async fn store_curr_tab_children_by_path(&mut self, path: &str) {
        let children = self.get_children(path).await;
        if let Some(children) = children {
            if !children.is_empty() {
                self.store_children(children).await;
                self.curr_tab_mut().list_state.select(Some(0));
            } else {
                self.set_tab_message("Node does not have children nodes".to_owned());
            }
        }
    }

    pub(crate) async fn store_node_data(&mut self) {
        let Some(ref zk) = self.zk else {
            return;
        };

        let _ = zk
            .get_data(&self.tab_full_resource_path(), false)
            .await
            .map(|(data, _)| {
                self.curr_tab_mut().node_data = NodeData::Raw(data);
            });
    }

    pub(crate) async fn create_node(&mut self) {
        let Some(ref zk) = self.zk else {
            "Failed to get zookeeper client".clone_into(&mut self.curr_tab_mut().message);
            return;
        };

        let res = zk
            .create(
                &self.curr_tab().node_path_buf,
                self.curr_tab().node_data_buf.clone().into_bytes(), //TODO: Avoid clonning
                Acl::open_unsafe().clone(),
                zookeeper_async::CreateMode::Persistent,
            )
            .await;
        match res {
            Ok(created_path) => {
                self.curr_tab_mut().message = format!("Node {created_path} created successfully")
            }
            Err(e) => self.curr_tab_mut().message = format!("Node creation failed : {e}"),
        }
    }

    pub(crate) async fn set_data(&mut self) {
        let data = mem::take(&mut self.curr_tab_mut().node_data_buf).into_bytes();

        let Some(ref zk) = self.zk else {
            "Failed to get zookeeper client".clone_into(&mut self.curr_tab_mut().message);
            return;
        };

        let res = zk
            .set_data(&self.tab_full_resource_path(), data, None)
            .await;
        match res {
            Ok(_) => {
                self.curr_tab_mut().message = format!(
                    "Node {} data successfully updated",
                    self.tab_full_resource_path()
                )
            }
            Err(e) => self.curr_tab_mut().message = format!("Node data update failed : {e}"),
        }
    }

    pub(crate) async fn delete_node(&mut self) {
        let Some(ref zk) = self.zk else {
            "Failed to get zookeeper client".clone_into(&mut self.curr_tab_mut().message);
            return;
        };
        let res = zk.delete(&self.tab_full_resource_path(), None).await;
        match res {
            Ok(_) => {
                self.curr_tab_mut().message = format!(
                    "Node {} successfully deleted",
                    self.tab_full_resource_path()
                )
            }
            Err(e) => self.curr_tab_mut().message = format!("Delete node failed : {e}"),
        }
    }
    pub(crate) async fn get_children(&self, path: &str) -> Option<Vec<String>> {
        let Some(ref zk) = self.zk else {
            return None;
        };
        zk.get_children(path, false).await.ok()
    }
}
