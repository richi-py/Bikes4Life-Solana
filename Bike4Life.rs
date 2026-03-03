use anchor_lang::prelude::*;

declare_id!("4Ww2MDrEqGjB7LJGBs1QTBVGrJuX21SL6WPTwKXQ34sF");

#[program]
pub mod tienda_bicicletas {
    use super::*;
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

    if context.accounts.tienda.owner != Pubkey::default() {
        msg!("La tienda ya existe");
        return Ok(());
    }

    let owner_id = context.accounts.owner.key();

    context.accounts.tienda.set_inner(Tienda {
        owner: owner_id,
        nombre: nombre.clone(),
        bicicletas: Vec::new(),
    });

    msg!("Tienda creada correctamente");

    Ok(())
}

    pub fn agregar_bicicleta(
        context: Context<NuevaBicicleta>,
        marca: String,
        modelo: String,
        precio: u64,
        categoria: String,
        stock: u16,
    ) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let bici = Bicicleta {
            marca,
            modelo,
            precio,
            categoria,
            stock,
            disponible: true,
        };

        context.accounts.tienda.bicicletas.push(bici);

        Ok(())
    }

    pub fn eliminar_bicicleta(context: Context<NuevaBicicleta>, modelo: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let bicicletas = &mut context.accounts.tienda.bicicletas;

        for i in 0..bicicletas.len() {
            if bicicletas[i].modelo == modelo {
                bicicletas.remove(i);
                msg!("Bicicleta {} eliminada!", modelo);
                return Ok(());
            }
        }

        Err(Errores::BicicletaNoExiste.into())
    }

    pub fn ver_bicicletas(context: Context<NuevaBicicleta>) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let tienda = &context.accounts.tienda;

        msg!("==================================");
        msg!("Tienda: {}", tienda.nombre);
        msg!("Total bicicletas: {}", tienda.bicicletas.len());
        msg!("==================================");

        for bici in &tienda.bicicletas {
            msg!("------------------------------");
            msg!("Marca: {}", bici.marca);
            msg!("Modelo: {}", bici.modelo);
            msg!("Precio: {}", bici.precio);
            msg!("Categoria: {}", bici.categoria);
            msg!("Stock: {}", bici.stock);
            msg!("Disponible: {}", bici.disponible);
        }

        Ok(())
    }

    pub fn modificar_bicicleta(
        context: Context<NuevaBicicleta>,
        modelo: String,
        nuevo_precio: u64,
        nuevo_stock: u16,
    ) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let bicicletas = &mut context.accounts.tienda.bicicletas;

        for i in 0..bicicletas.len() {
            if bicicletas[i].modelo == modelo {
                bicicletas[i].precio = nuevo_precio;
                bicicletas[i].stock = nuevo_stock;
                msg!("Bicicleta {} modificada!", modelo);
                return Ok(());
            }
        }

        Err(Errores::BicicletaNoExiste.into())
    }

    pub fn alternar_disponibilidad(context: Context<NuevaBicicleta>, modelo: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let bicicletas = &mut context.accounts.tienda.bicicletas;

        for i in 0..bicicletas.len() {
            if bicicletas[i].modelo == modelo {
                bicicletas[i].disponible = !bicicletas[i].disponible;
                msg!("Disponibilidad cambiada para {}", modelo);
                return Ok(());
            }
        }

        Err(Errores::BicicletaNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("No eres el propietario de la tienda")]
    NoEresElOwner,
    #[msg("La bicicleta no existe")]
    BicicletaNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(20)]
    pub bicicletas: Vec<Bicicleta>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Bicicleta {
    #[max_len(40)]
    pub marca: String,

    #[max_len(40)]
    pub modelo: String,

    pub precio: u64,

    #[max_len(40)]
    pub categoria: String,

    pub stock: u16,

    pub disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevaBicicleta<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
