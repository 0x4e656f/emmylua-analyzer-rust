use lsp_types::ClientInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClientId {
    VSCode,
    Intellij,
    Neovim,
    Other,
}

impl Default for ClientId {
    fn default() -> Self {
        ClientId::Other
    }
}

#[allow(unused)]
impl ClientId {
    pub fn is_vscode(&self) -> bool {
        matches!(self, ClientId::VSCode)
    }

    pub fn is_intellij(&self) -> bool {
        matches!(self, ClientId::Intellij)
    }

    pub fn is_neovim(&self) -> bool {
        matches!(self, ClientId::Neovim)
    }

    pub fn is_other(&self) -> bool {
        matches!(self, ClientId::Other)
    }
}

pub fn get_client_id(client_info: &Option<ClientInfo>) -> ClientId {
    match client_info {
        Some(info) => match info.name.as_str() {
            "Visual Studio Code" => ClientId::VSCode,
            "IntelliJ" => ClientId::Intellij,
            "Neovim" | "coc.nvim" => ClientId::Neovim,
            "Cursor" | "Windsurf" | "Trae" => ClientId::VSCode,
            _ => ClientId::Other,
        },
        None => ClientId::Other,
    }
}
