#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract02 { 
    use ink::prelude::vec::Vec;
    use ink::prelude::{format, string::String};
    use ink::storage::Mapping;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Contract02 { 
        /// Stores a single `bool` value on the storage.
        value: bool,
        usuarios: Mapping<AccountId, Usuario>,
    }

    //Struct usuario
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone)]
    pub struct Usuario{
        nombre:String,
        apellido:String,
        email:String,
        id:AccountId,
        rol: Rol,
    }
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone)]
    pub enum Rol {
        Comprador,
        Vendedor,
        Ambos,
    }

    /*
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Producto{
        nombre: String,
        descripcion: String,
        precio: f64,
        stock: u64,
        publicador: AccountId,
    }
    */

    /* ORDEN DE COMPRA 
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]

    pub struct OrdenDeCompra {
        productos: Vec<Producto>,
        id_compra: u64 o String o qué ????
        direccion_de_envio: String,
    } 
    
    */

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]

    pub enum ErrorSistema {
        UsuarioYaRegistrado,
        UsuarioNoExiste,
    
    }

    impl Contract02 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value, usuarios: Vec::new() }
        }

        #[ink(message)]
        pub fn registrar_usuario(&mut self, nombre:String, apellido:String, email:String, rol:Rol) -> Result<(), ErrorSistema> {
            self._registrar_usuario(nombre, apellido, email, rol)?;
            Ok(())
        }

        fn _registrar_usuario(&mut self, nombre:String, apellido:String, email:String, rol:Rol) -> Result<(), ErrorSistema>{
            let id = self.env().caller();
            
            // Chequear que el usuario a registrar no exista en el sistema. (Solo registrar usuarios nuevos)
            if self.usuarios.iter().any(|x| x.id == id) {
                return Err(ErrorSistema::UsuarioYaRegistrado);
            }
            
            self.usuarios.try_insert(id, Usuario {nombre, apellido, email, id, rol});
            Ok(())
        }
        
        /// Devuelve la lista de los usuarios
        #[ink(message)]
        pub fn get_users(&self) -> Vec<Usuario>{//Result por si la lista de usuarios está vacia???'
            self.usuarios.clone()
        }
        
        /// Devuelve un usuario en particular
        #[ink(message)]
        pub fn get_user(&self) -> Result<Usuario, ErrorSistema>{//result
            self._get_user() 
        }

        fn _get_user(&self)-> Result<Usuario, ErrorSistema>{
            let _caller = self.env().caller(); //Se busca con el AccountId de la cuenta asociada.

            if let Some(user) = self.usuarios.iter().find(|x| x.id == _caller){
                Ok(user.clone())
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }

        //  #[ink(message)]
        //  pub fn modificar_rol(&self, nuevo_rol:Rol)->Result<(), ErrorSistema>{
        //     self._modificar_rol(nuevo_rol)?
        //  }

        //  fn _modificar_rol(&mut self, nuevo_rol:Rol) {
        //     if let Some(user) = self.usuarios.get_mut(self.env().caller()){
        //         user.rol = nuevo_rol;
        //         return Ok(());
        //     }
        //     return Err(ErrorSistema::UsuarioNoExiste);
        //  }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /*
        /// Determina si un usuario tiene el rol de vendedor.
        fn usuario_puede_publicar(&self, id: AccountId) -> bool {
        }

        fn _usuario_puede_publicar(&self, id: AccountId) -> bool {
            
        }
        */

        
        //VERIFICADORES
       /*  #[ink(message)]
        pub fn es_vendedor(&self) -> bool {
            let caller = self.env().caller();
            if let Some(user) = self.usuarios.get(caller) {
                match user.rol {
                    Rol::Vendedor | Rol::Ambos => true,
                    _ => false,
                }
            } else {
                false // Si el usuario no existe, no es vendedor.
            }
        }

        #[ink(message)]
        pub fn es_comprador(&self) -> bool {
            let caller = self.env().caller();
            if let Some(user) = self.usuarios.get(caller) {
                match user.rol {
                    Rol::Comprador | Rol::Ambos => true,
                    _ => false,
                }
            } else {
                false // Si el usuario no existe, no es comprador.
            }
        } */

        

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        // Librerías para generar un AccountId
        use sp_core::crypto::AccountId32;
        use rand::rngs::OsRng;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let contract02 = Contract02::default();
            assert_eq!(contract02.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut contract02 = Contract02::new(false);
            assert_eq!(contract02.get(), false);
            contract02.flip();
            assert_eq!(contract02.get(), true);
        }
        
        /*
        fn generar_account_id(&self) -> AccountId {
            let mut rng = OsRng {};
            let keypair: sp_core::sr25519::Pair = sp_core::sr25519::Pair::generate_with(&mut rng);
            keypair.public().into()
        }
        */

        //Test de devuelve un usurio
        #[ink::test]
        fn test_obtener_usuario() {
            let usuario = Usuario {
                nombre: "Tomás".to_string(),
                apellido: "Perez".to_string(),
                email: "b@mail.com".to_string(),
                id: "bChVMyjX9ezHYc4TSknGWKibZxLQJcSfqmmXqSUZecejjXZ".to_string(), //generar_account_id()
                rol: Rol::Comprador,
            };
            let mut contract02 = Contract02::new(false);

            test::set_caller::<ink::env::DefaultEnvironment>(usuario.id.clone()); 
            //establece el caller del usuario
            
            assert!(contract02.registrar_usuario(usuario.nombre.clone(), usuario.apellido.clone(), usuario.email.clone(), usuario.rol.clone()).is_ok());
            //Si cumple el assert sigue con el test

            let user = contract02.get_user();
            //Devuelve Ok(UsuarioBuscado) si el usuario se encuentra en la lista de usuarios registrados
            assert_eq!(usuario, Ok(user));
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = Contract02Ref::default();

            // When
            let contract = client
                .instantiate("contract02", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Contract02>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = Contract02Ref::new(false);
            let contract = client
                .instantiate("contract02", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Contract02>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
