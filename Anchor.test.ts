describe("Tienda Bicicletas", () => {

  it("Crear tienda y leer datos", async () => {

    const [tiendaPda] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("tienda"),
        pg.wallet.publicKey.toBuffer(),
      ],
      pg.program.programId
    );


    console.log("DIRECCIÓN DE LA TIENDA (PDA):", tiendaPda.toBase58());

    const txHash = await pg.program.methods
      .crearTienda("Bike4Life")
      .accounts({
        owner: pg.wallet.publicKey,
        tienda: tiendaPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Tx:", txHash);

    await pg.connection.confirmTransaction(txHash);

    console.log("Tienda creada correctamente");

    const tienda = await pg.program.account.tienda.fetch(tiendaPda);

    console.log("Datos on-chain:");
    console.log("Owner:", tienda.owner.toString());
    console.log("Nombre:", tienda.nombre);
    
    const bicisLegibles = tienda.bicicletas.map(b => ({
      ...b,
      precio: b.precio.toString() 
    }));

    console.log("Bicicletas:", bicisLegibles);

    if (tienda.nombre !== "Bike4Life") {
      throw new Error("El nombre no coincide");
    }

  });

});
