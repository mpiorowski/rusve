use crate::proto::utils_service_server::UtilsService;
use crate::proto::{Count, Email, Empty, File, Id, Page};
use crate::MyService;
use anyhow::Result;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl UtilsService for MyService {
    type GetEmailsByTargetIdStream = ReceiverStream<Result<Email, Status>>;
    type GetFilesByTargetIdStream = ReceiverStream<Result<File, Status>>;
    type GetFileByIdStream = ReceiverStream<Result<File, Status>>;
    type UploadFileStream = ReceiverStream<Result<File, Status>>;

    async fn count_emails_by_target_id(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Count>, Status> {
        crate::email_service::count_emails_by_target_id(&self.pool, request).await
    }

    async fn get_emails_by_target_id(
        &self,
        request: Request<Page>,
    ) -> Result<Response<Self::GetEmailsByTargetIdStream>, Status> {
        crate::email_service::get_emails_by_target_id(&self.pool, request).await
    }

    async fn send_email(&self, request: Request<Email>) -> Result<Response<Email>, Status> {
        crate::email_service::send_email(&self.env, &self.pool, request).await
    }

    async fn count_files_by_target_id(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Count>, Status> {
        crate::file_service::count_files_by_target_id(&self.pool, request).await
    }

    async fn get_files_by_target_id(
        &self,
        request: Request<Page>,
    ) -> Result<Response<Self::GetFilesByTargetIdStream>, Status> {
        crate::file_service::get_files_by_target_id(&self.pool, request).await
    }

    async fn get_file_by_id(
        &self,
        request: Request<Id>,
    ) -> Result<Response<Self::GetFileByIdStream>, Status> {
        crate::file_service::get_file_by_id(&self.env, &self.pool, request).await
    }

    async fn upload_file(
        &self,
        request: Request<tonic::Streaming<File>>,
    ) -> Result<Response<Self::UploadFileStream>, Status> {
        crate::file_service::upload_file(&self.env, &self.pool, request).await
    }

    async fn delete_file_by_id(&self, request: Request<Id>) -> Result<Response<Empty>, Status> {
        crate::file_service::delete_file_by_id(&self.env, &self.pool, request).await
    }
}
