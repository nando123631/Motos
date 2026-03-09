// Importa el prelude de Anchor (macros y utilidades principales)
use anchor_lang::prelude::*;

// Declara el ID del programa en Solana
declare_id!("DEgau7JsQKcvsrgvjyKDUtFN6sZSSyiiZcNZBdRbTvgV");

// Define el módulo principal del programa
#[program]
pub mod tienda_motos {
    use super::*;

    // Función para crear una tienda
    pub fn crear_tienda(context: Context<NuevaTienda>, marca: String) -> Result<()> {
        // Obtiene la clave pública del owner
        let owner_id = context.accounts.owner.key();
        // Muestra en logs el owner
        msg!("Owner id: {}", owner_id);

        // Inicializa un vector vacío de productos
        let productos: Vec<Moto> = Vec::new();

        // Asigna los valores a la cuenta tienda
        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            marca,
            productos,
        });

        Ok(())
    }

    // Función para agregar un producto a la tienda
    pub fn agregar_producto(
        context: Context<NuevoProducto>,
        marca: String,
        precio: u16,
    ) -> Result<()> {
        // Verifica que el firmante sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Crea una nueva moto
        let moto = Moto {
            marca,
            precio,
            disponible: true,
        };

        // Agrega la moto al vector de productos
        context.accounts.tienda.productos.push(moto);

        Ok(())
    }

    // Función para eliminar un producto por marca
    pub fn eliminar_producto(context: Context<NuevoProducto>, marca: String) -> Result<()> {
        // Verifica que el firmante sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtiene referencia mutable a los productos
        let productos = &mut context.accounts.tienda.productos;

        // Busca el producto por marca y lo elimina
        for i in 0..productos.len() {
            if productos[i].marca == marca {
                productos.remove(i);
                msg!("Producto {} eliminado!", marca);
                return Ok(());
            }
        }

        // Si no existe, devuelve error
        Err(Errores::ProductoNoExiste.into())
    }

    // Función para ver productos registrados
    pub fn ver_productos(context: Context<NuevoProducto>) -> Result<()> {
        // Verifica que el firmante sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Muestra la lista de productos en logs
        msg!(
            "Lista de productos: {:#?}",
            context.accounts.tienda.productos
        );
        Ok(())
    }

    // Función para alternar disponibilidad de un producto
    pub fn alternar_disponibilidad(context: Context<NuevoProducto>, marca: String) -> Result<()> {
        // Verifica que el firmante sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtiene referencia mutable a los productos
        let productos = &mut context.accounts.tienda.productos;

        // Busca el producto y cambia su estado
        for i in 0..productos.len() {
            let estado = productos[i].disponible;

            if productos[i].marca == marca {
                let nuevo_estado = !estado;
                productos[i].disponible = nuevo_estado;

                msg!(
                    "El producto {} ahora tiene disponibilidad: {}",
                    marca,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        // Si no existe, devuelve error
        Err(Errores::ProductoNoExiste.into())
    }

    // Función para contar productos
    pub fn total_productos(context: Context<NuevoProducto>) -> Result<()> {
        // Calcula el total de productos
        let total = context.accounts.tienda.productos.len();

        // Muestra el total en logs
        msg!("La tienda tiene {} productos registrados", total);

        Ok(())
    }
}

// Definición de errores personalizados
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    #[msg("El producto no existe")]
    ProductoNoExiste,
}

// Definición de la cuenta Tienda
#[account]
#[derive(InitSpace)]
pub struct Tienda {
    owner: Pubkey, // Propietario de la tienda

    #[max_len(60)]
    marca: String, // Marca de la tienda

    #[max_len(10)]
    productos: Vec<Moto>, // Lista de motos
}

// Definición de la estructura Moto
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Moto {
    #[max_len(60)]
    marca: String, // Marca de la moto

    precio: u16,   // Precio de la moto

    disponible: bool, // Estado de disponibilidad
}

// Contexto para crear una nueva tienda
#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // Firmante que paga la creación

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>, // Cuenta de la tienda

    pub system_program: Program<'info, System>, // Programa del sistema
}

// Contexto para interactuar con productos
#[derive(Accounts)]
pub struct NuevoProducto<'info> {
    pub owner: Signer<'info>, // Firmante

    #[account(mut)]
    pub tienda: Account<'info, Tienda>, // Cuenta de la tienda
}
