// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct UserId(String);

#[derive(Clone, Debug)]
pub struct RoleId(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Create,
    Read,
    List,
    Modify,
    Reconcile,
    Sign,
    Close,
    Delete,
}

#[derive(Clone, Debug)]
pub enum Resource {
    Account,
    Commodity,
    Customer,
    Journal,
    Ledger,
    Supplier,
    Transaction,
    User,
}

pub trait Authenticator {
    fn user_has_role(&self, user: UserId, role: RoleId) -> bool;

    fn role_can_perform(&self, role: RoleId, action: Action, resource: Resource) -> bool;

    fn user_can_perform(&self, user: UserId, action: Action, resource: Resource) -> bool;

    #[inline]
    fn user_can_create(&self, user: UserId, resource: Resource) -> bool {
        self.user_can_perform(user, Action::Create, resource)
    }

    #[inline]
    fn user_can_modify(&self, user: UserId, resource: Resource) -> bool {
        self.user_can_perform(user, Action::Create, resource)
    }

    #[inline]
    fn user_can_reconcile(&self, user: UserId, resource: Resource) -> bool {
        self.user_can_perform(user, Action::Create, resource)
    }

    #[inline]
    fn user_can_sign(&self, user: UserId, resource: Resource) -> bool {
        self.user_can_perform(user, Action::Create, resource)
    }

    #[inline]
    fn user_can_close(&self, user: UserId, resource: Resource) -> bool {
        self.user_can_perform(user, Action::Create, resource)
    }

    #[inline]
    fn user_can_delete(&self, user: UserId, resource: Resource) -> bool {
        self.user_can_perform(user, Action::Create, resource)
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
