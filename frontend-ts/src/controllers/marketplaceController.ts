import { ApiRustClient } from "../services/apiRustClient";
import { CouchDbSyncService } from "../services/couchdbSync";

export class MarketplaceController {
  private apiClient: ApiRustClient;
  private syncService: CouchDbSyncService;

  constructor() {
    this.apiClient = new ApiRustClient();
    this.syncService = new CouchDbSyncService();
  }

  async inicializarSistemaCliente() {
    console.log("--- Iniciando Cliente / Orquestador TypeScript ---");
    await this.syncService.inicializarBaseLocal();
    await this.syncService.iniciarReplicacionContinua();
  }

  async ejecutarPruebaFlujoCompleto() {
    try {
      console.log(
        "\n1. Registrando PYME Proveedora a través del Backend en Rust...",
      );
      const pymeRes = await this.apiClient.registrarPyme({
        nombre_comercial: "Panadería Artesanal San José",
        nit: "0614-123456-101-2",
        nrc: "123456-7",
        sector: "Panadería y Alimentos",
        perfil_verificado: true,
        calificacion: 4.8,
      });
      console.log("Resultado Registro PYME:", pymeRes);

      console.log("\n2. Creando ítem en el catálogo consolidado...");
      const productoRes = await this.apiClient.crearProducto({
        pyme_id: "pyme_test_01",
        nombre: "Pack 50 Baguettes Integrales (Institucional)",
        descripcion: "Pan empacado al vacío para entrega en cadena de hoteles",
        precio_unitario: 45.5,
        stock_disponible: 200,
        unidad_medida: "Paquete",
      });
      console.log("Resultado Producto:", productoRes);

      console.log("\n3. Procesando Orden Consolidada...");
      const ordenRes = await this.apiClient.crearOrden({
        comprador_institucional: "Hotel Gran Plaza San Salvador",
        items: [
          {
            producto_id: "prod_test_01",
            pyme_id: "pyme_test_01",
            cantidad: 4,
            subtotal: 182.0,
          },
        ],
      });
      console.log("Resultado Orden Consolidada:", ordenRes);
      const ordenId = ordenRes.orden_id;

      console.log(
        "\n4. Emitiendo Facturación Electrónica (DTE) tipo 01 (Factura)...",
      );
      const dteRes = await this.apiClient.emitirDte(ordenId, "01");
      console.log("Resultado Emisión DTE:", dteRes);
    } catch (error: any) {
      console.error(
        "Error durante la ejecución del flujo del marketplace:",
        error.response?.data || error.message,
      );
    }
  }
}
