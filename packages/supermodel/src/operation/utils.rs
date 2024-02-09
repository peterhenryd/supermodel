use crate::dialect::Dialect;
use crate::executor::Executor;
use crate::operation::Operation;

pub trait Utils<D: Dialect>: Operation<D> {
    async fn execute(&self, executor: &mut impl Executor<D>) -> Result<Self::Output, D::Error>;
}

impl<D: Dialect, O: Operation<D>> Utils<D> for O {
    #[inline]
    async fn execute(&self, executor: &mut impl Executor<D>) -> Result<Self::Output, D::Error> {
        let query = self.as_query();
        executor.execute_query::<Self::Output>(query).await
    }
}