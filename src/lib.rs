use az_dioxus_admin_shell::ApplicationAccountItem;
use dill::CatalogBuilder;

#[derive(Debug)]
pub struct AccountPlugin;

impl az_dioxus_admin_shell::ApplicationAccountPlugin for AccountPlugin {
    fn items(&self) -> Vec<ApplicationAccountItem> {
        vec![
            ApplicationAccountItem {
                id: "profile".into(),
                label: "个人资料".into(),
                icon: Some("profile".into()),
                page_id: Some("profile".into()),
                required_permission: None,
                destructive: false,
            },
            ApplicationAccountItem {
                id: "settings".into(),
                label: "设置中心".into(),
                icon: Some("settings".into()),
                page_id: Some("settings".into()),
                required_permission: None,
                destructive: false,
            },
            ApplicationAccountItem {
                id: "marketplace".into(),
                label: "插件市场".into(),
                icon: Some("settings".into()),
                page_id: Some("marketplace".into()),
                required_permission: Some("plugin:manage".into()),
                destructive: false,
            },
            ApplicationAccountItem {
                id: "logout".into(),
                label: "退出系统".into(),
                icon: Some("logout".into()),
                page_id: None,
                required_permission: None,
                destructive: true,
            },
        ]
    }
}

pub fn register(builder: &mut CatalogBuilder) {
    builder
        .add_value(AccountPlugin)
        .bind::<dyn az_dioxus_admin_shell::ApplicationAccountPlugin, AccountPlugin>();
}
