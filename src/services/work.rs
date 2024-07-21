use std::sync::Arc;

use crate::accessors::db::UserDbAccessor;
use crate::accessors::net::{DailyNetAccessor, UserNetAccessor};

pub(crate) struct UserNetService {
    user_net_accessor: Arc<UserNetAccessor>,
    daily_net_accessor: Arc<DailyNetAccessor>,
    user_db_accessor: Arc<UserDbAccessor>,
}
impl UserNetService {
    pub(crate) fn new(user_net_accessor: Arc<UserNetAccessor>, daily_net_accessor: Arc<DailyNetAccessor>, user_db_accessor: Arc<UserDbAccessor>) -> Self {
        Self {
            user_net_accessor,
            daily_net_accessor,
            user_db_accessor,
        }
    }
    pub(crate) async fn user_task(&self) {
        let mut run = true;
        while run {
            match self.user_net_accessor.register_user().await {
                Ok(user) => {
                    self.user_db_accessor.add_user(user).await.unwrap();
                }
                Err(err) => {
                    run = false;
                }
            }
        }
        let users = self.user_db_accessor.get_users().await.unwrap();
        for user in users {
            self.daily_net_accessor.daily_sign(user.id()).await.unwrap();
            self.daily_net_accessor.daily_work(&1, user.id()).await.unwrap();
            self.daily_net_accessor.daily_work(&4, user.id()).await.unwrap();
            self.daily_net_accessor.daily_work(&6, user.id()).await.unwrap();
            self.daily_net_accessor.daily_work(&7, user.id()).await.unwrap();
        }
    }
}
