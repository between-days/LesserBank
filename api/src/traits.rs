// shared trait definitions

use crate::error::RepoError;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait RepoFind<T: 'static + Sync + Send, Q: 'static + Sync + Send>:
    'static + Sync + Send
{
    fn find(&self, query: Q) -> Result<Vec<T>, RepoError>;
}

#[cfg_attr(test, automock)]
pub trait RepoCreate<T: 'static + Sync + Send, N: 'static + Sync + Send>:
    'static + Sync + Send
{
    fn create(&self, new: N) -> Result<T, RepoError>;
}

#[cfg_attr(test, automock)]
pub trait RepoGetById<T: 'static + Sync + Send>: 'static + Sync + Send {
    fn get_by_id(&self, id: i32) -> Result<T, RepoError>;
}

#[cfg_attr(test, automock)]
pub trait RepoDeleteById<T: 'static + Sync + Send>: 'static + Sync + Send {
    fn delete_by_id(&self, id: i32) -> Result<(), RepoError>;
}
