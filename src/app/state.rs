#[derive(Debug, Default, PartialEq)]
pub enum AppState {
    #[default]
    EstablishingConnection,
    EditingConnection,
    Tab,
    ReadNodeData,
    EditCreateNodePath,
    EditCreateNodeData,
    EditNodeData,
    DeleteNode,
    ConfirmDelete,
}
