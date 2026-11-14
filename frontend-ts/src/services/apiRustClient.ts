import axios, { AxiosInstance } from "axios";
import { config } from "../config/database";

export class ApiRustClient {
  private client: AxiosInstance;

  constructor() {
    this.client = axios.create({
      baseURL: config.rustApiUrl,
      timeout: 10000,
      headers: {
        "Content-Type": "application/json",
      },
    });
  }

  async registrarPyme(pymeData: any) {
    const response = await this.client.post("/api/pymes", pymeData);
    return response.data;
  }

  async listarPymes() {
    const response = await this.client.get("/api/pymes");
    return response.data;
  }

  async crearProducto(productoData: any) {
    const response = await this.client.post("/api/catalogo", productoData);
    return response.data;
  }

  async listarCatalogo() {
    const response = await this.client.get("/api/catalogo");
    return response.data;
  }

  async crearOrden(ordenData: any) {
    const response = await this.client.post("/api/ordenes", ordenData);
    return response.data;
  }

  async emitirDte(ordenId: string, tipoDte: string) {
    const response = await this.client.post("/api/dte/emitir", {
      orden_id: ordenId,
      tipo_dte: tipoDte,
    });
    return response.data;
  }
}
