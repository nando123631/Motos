// No imports needed: web3, anchor, pg and more are globally available

describe("Tienda de Motos", () => {
  it("crear tienda", async () => {
    // Generar keypair para la nueva tienda
    const tiendaKp = new web3.Keypair();

    // Definir la marca de la tienda
    const marcaTienda = "MotoRacing";

    // Enviar transacción para crear la tienda
    const txHash = await pg.program.methods
      .crearTienda(marcaTienda)
      .accounts({
        owner: pg.wallet.publicKey,
        tienda: tiendaKp.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([tiendaKp])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' para ver los logs`);

    // Confirmar transacción
    await pg.connection.confirmTransaction(txHash);

    // Fetch de la cuenta tienda creada
    const tienda = await pg.program.account.tienda.fetch(tiendaKp.publicKey);

    console.log("Datos de la tienda en cadena:", tienda);

    // Verificar que la marca coincide con la que enviamos
    assert.equal(tienda.marca, marcaTienda);

    // Verificar que el owner es el wallet actual
    assert.equal(tienda.owner.toString(), pg.wallet.publicKey.toString());

    // Verificar que inicialmente no hay productos
    assert.equal(tienda.productos.length, 0);
  });
});
