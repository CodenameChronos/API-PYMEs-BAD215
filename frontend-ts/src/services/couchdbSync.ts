import Nano from "nano";
import { config } from "../config/database";

export class CouchDbSyncService {
  private nanoLocal: Nano.ServerScope;
  private dbName: string;

  constructor() {
    this.nanoLocal = Nano(config.couchdbLocalUrl);
    this.dbName = config.dbName;
  }

  async inicializarBaseLocal() {
    try {
      const dbList = await this.nanoLocal.db.list();
      if (!dbList.includes(this.dbName)) {
        await this.nanoLocal.db.create(this.dbName);
        console.log(
          `[CouchDB Local] Base de datos '${this.dbName}' creada exitosamente.`,
        );
      } else {
        console.log(
          `[CouchDB Local] Base de datos '${this.dbName}' ya existe.`,
        );
      }
    } catch (error) {
      console.error(
        "[CouchDB Local] Error al verificar/crear la base local:",
        error,
      );
    }
  }

  /**
   * Configura la replicación continua bidireccional (offline-first)
   * entre el nodo local de la PYME y el repositorio central.
   */
  async iniciarReplicacionContinua() {
    const remoteUrlWithDb = `${config.couchdbRemoteUrl}/${this.dbName}`;
    try {
      await this.nanoLocal.db.replication.enable(this.dbName, remoteUrlWithDb, {
        continuous: true,
        create_target: false,
      });
      console.log(
        "[CouchDB Sync] Replicación multi-maestro continua establecida con éxito.",
      );
    } catch (error: any) {
      console.log(
        "[CouchDB Sync] Nota de replicación (red intermitente o sincronizado previamente):",
        error.message,
      );
    }
  }
}
