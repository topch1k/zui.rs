#[derive(Debug, Default, PartialEq)]
pub enum AppState {
    #[default]
    EstablishingConnection,
    EditingConnection,
    Tab,
}

#[derive(Debug, Default, PartialEq)]
pub enum TabState {
    #[default]
    Tab,
    ReadNodeData,
    EditCreateNodePath,
    EditCreateNodeData,
    EditNodeData,
    DeleteNode,
    ConfirmDelete,
}
