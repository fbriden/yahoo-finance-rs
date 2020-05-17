/// Used in conjunction with Serde to create good public structures
macro_rules! ez_serde {
   ($name:ident$(< $( $lt:lifetime ),+ >)? { $($(#[$m:meta])? $field:ident: $t:ty),* } ) => {
      #[derive(Clone, Deserialize)]
      #[serde(rename_all(deserialize = "camelCase"))]
      pub struct $name$(< $($lt),* >)? {
         $($(#[$m])? pub $field: $t),*
      }
   };
   ($name:ident { $($(#[$m:meta])? $field:ident: $t:ty),* } ) => {
      #[derive(Clone, Deserialize)]
      #[serde(rename_all(deserialize = "camelCase"))]
      pub struct $name {
         $($(#[$m])? pub $field: $t),*
      }
   }
}
