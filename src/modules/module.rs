use pumpkin::command::tree::CommandTree;
use pumpkin::server::Server;
use pumpkin_util::permission::Permission;
use std::collections::HashSet;
use std::sync::Arc;

/// A trait representing a generic module with an `enabled` status.
///
/// This trait is intended to be implemented by types that represent
/// a module or component that can be enabled or disabled. The implementation
/// of the `enabled` method provides a way to query the current status of the module.
pub trait Module {
    /// Returns `true` if the module is enabled, `false` otherwise.
    fn enabled(&self) -> bool;

    /// Returns a set of `CommandTree`s associated with the current instance.
    ///
    /// # Returns
    /// - `HashSet<CommandTree>`: Returns an empty set in the default implementation.
    fn cmds(&self) -> HashSet<CommandTree> {
        HashSet::new()
    }

    /// Returns a set of `Permission`s associated with the current instance.
    ///
    /// # Returns
    /// - `HashSet<Permission>`: Returns an empty set in the default implementation.
    fn perms(&self) -> HashSet<Permission> {
        HashSet::new()
    }

    //TODO docs
    async fn register(&self, ctx: Arc<Server>) {
        ctx.register_permission(todo!("perm here")).await.unwrap();
        ctx.register_command(todo!("cmd here"), todo!("perm here"))
            .await;
    } //TODO implementation
}
