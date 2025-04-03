// crud_macro/src/lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CRUD)]
pub fn derive_crud(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl CRUD for #name {
            async fn create(
                db: &sea_orm::DatabaseConnection,
                data: sea_orm::ActiveModelTrait::<Self>,
            ) -> sea_orm::Result<Self> 
            where
                Self: sea_orm::EntityTrait,
                <Self as sea_orm::EntityTrait>::ActiveModel: From<Self>,
            {
                data.insert(db).await
            }

            async fn find_by_id(
                db: &sea_orm::DatabaseConnection,
                id: <Self::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType,
            ) -> sea_orm::Result<Option<Self>> 
            where
                Self: sea_orm::EntityTrait,
            {
                <Self as sea_orm::EntityTrait>::find_by_id(id).one(db).await
            }

            async fn find_all(
                db: &sea_orm::DatabaseConnection,
            ) -> sea_orm::Result<Vec<Self>> 
            where
                Self: sea_orm::EntityTrait,
            {
                <Self as sea_orm::EntityTrait>::find().all(db).await
            }

            async fn update(
                db: &sea_orm::DatabaseConnection,
                data: sea_orm::ActiveModelTrait::<Self>,
            ) -> sea_orm::Result<Self> 
            where
                Self: sea_orm::EntityTrait,
            {
                data.update(db).await
            }

            async fn delete(
                db: &sea_orm::DatabaseConnection,
                data: sea_orm::ActiveModelTrait::<Self>,
            ) -> sea_orm::Result<sea_orm::DeleteResult> 
            where
                Self: sea_orm::EntityTrait,
            {
                data.delete(db).await
            }
        }
    };

    TokenStream::from(expanded)
}

