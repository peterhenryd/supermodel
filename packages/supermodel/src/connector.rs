use crate::dialect::Dialect;
use crate::executor::Executor;

pub trait Connector<D: Dialect> {

}

pub trait Connection<D: Dialect>: Connector<D> + Executor<D> {
}

pub trait Pool<D: Dialect>: Connector<D> + Clone {
    type Executor: Executor<D>;

    async fn acquire(&self) -> Result<Self::Executor, D::Error>;
}