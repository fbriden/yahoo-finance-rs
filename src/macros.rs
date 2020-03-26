#[macro_export]
macro_rules! pub_json {
   ($name:ident { $($(#[$m:meta])? $field:ident: $t:ty),* } ) => {
      #[derive(Debug, Clone, Deserialize)]
      #[serde(rename_all(deserialize = "camelCase"))]
      pub(crate) struct $name {
         $($(#[$m])? pub $field: $t),*
      }
   }
}