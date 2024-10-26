use crate::{commands::Command, workspaces::Workspace, Result};
use std::future::Future;
use uuid::Uuid;

pub trait Ided {
    fn id(&self) -> Uuid;
}

pub trait Import {
    type Entity;

    fn import(&self, entity: Self::Entity) -> impl Future<Output = Result<Self::Entity>>;
}

pub trait Iterate {
    type Entity;

    fn iterate<M, MR>(&self, map_fn: M) -> impl Future<Output = Result<()>>
    where
        M: Fn(Vec<Self::Entity>) -> MR,
        MR: Future<Output = Result<()>>;
}

pub trait ListByIds {
    type Entity;

    fn list_by_ids(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Self::Entity>>>;
}

pub trait Update {
    type Entity;

    fn update(&self, entity: Self::Entity) -> impl Future<Output = Result<Self::Entity>>;
}

pub struct BackupOperation<'a, LP, RP, T>
where
    LP: Iterate<Entity = T>,
    RP: Import<Entity = T> + ListByIds<Entity = T> + Update<Entity = T>,
    T: PartialEq + Ided,
{
    pub local_provider: &'a LP,
    pub remote_provider: &'a RP,
}

impl<'a, LP, RP, T> BackupOperation<'a, LP, RP, T>
where
    LP: Iterate<Entity = T>,
    RP: Import<Entity = T> + ListByIds<Entity = T> + Update<Entity = T>,
    T: PartialEq + Ided,
{
    pub async fn execute(&self) -> Result<()> {
        self.local_provider
            .iterate(|locals| async {
                let remotes = self.list_remotes(&locals).await?;

                for local in locals {
                    let remote = remotes.iter().find(|r| r.id() == local.id());

                    let Some(remote) = remote else {
                        self.remote_provider.import(local).await?;
                        continue;
                    };

                    if &local != remote {
                        self.remote_provider.update(local).await?;
                    }
                }

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn list_remotes(&self, locals: &[T]) -> Result<Vec<T>> {
        self.remote_provider
            .list_by_ids(locals.iter().map(|c| c.id()).collect())
            .await
    }
}

impl Ided for Command {
    fn id(&self) -> Uuid {
        self.id().unwrap()
    }
}

impl Ided for Workspace {
    fn id(&self) -> Uuid {
        self.id().unwrap()
    }
}
