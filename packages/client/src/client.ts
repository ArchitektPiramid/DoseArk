import type {
  BackupFile,
  BackupList,
  Brand,
  CreateBrandInput,
  CreateDoseLogInput,
  CreateInventoryLotInput,
  CreateNutrientInput,
  CreateProductInput,
  CreateStorageLocationInput,
  DailyIntakeReport,
  DatabaseOverview,
  DatabaseTableStats,
  ProfileValidation,
  RuntimeConfiguration,
  DoseLog,
  DoseLogListInput,
  DoseLogNutrient,
  DoseLogPage,
  InventoryLot,
  Nutrient,
  NutrientTrendReport,
  Product,
  ProductNutrient,
  SaveProductNutrientInput,
  StockMovement,
  StorageLocation,
  SystemHealth,
  UpdateBrandInput,
  UpdateDoseLogInput,
  UpdateInventoryLotInput,
  UpdateNutrientInput,
  UpdateProductInput,
  UpdateStorageLocationInput,
} from "@doseark/types";
import * as adminApi from "./api/admin";
import * as brandsApi from "./api/brands";
import * as dosingApi from "./api/dosing";
import * as inventoryApi from "./api/inventory";
import * as nutrientsApi from "./api/nutrients";
import * as productsApi from "./api/products";
import * as reportsApi from "./api/reports";
import * as storageLocationsApi from "./api/storage-locations";
import * as systemApi from "./api/system";
import type { DoseArkRemoteClient, NutrientTrendReportInput } from "./contracts";
import type { DoseArkClientOptions } from "./events";
import { HttpTransport } from "./transport";

export class DoseArkHttpClient implements DoseArkRemoteClient {
  private readonly transport: HttpTransport;

  constructor(options: DoseArkClientOptions) {
    this.transport = new HttpTransport(options);
  }

  setBaseUrl(baseUrl: string): void {
    this.transport.setBaseUrl(baseUrl);
  }

  getBaseUrl(): string {
    return this.transport.getBaseUrl();
  }

  health(timeoutMs = 5_000, healthPath = "/api/health"): Promise<SystemHealth> {
    return systemApi.getSystemHealth(this.transport, timeoutMs, healthPath);
  }

  listBrands(): Promise<Brand[]> {
    return brandsApi.listBrands(this.transport);
  }

  createBrand(input: CreateBrandInput): Promise<Brand> {
    return brandsApi.createBrand(this.transport, input);
  }

  updateBrand(id: string, input: UpdateBrandInput): Promise<Brand> {
    return brandsApi.updateBrand(this.transport, id, input);
  }

  listProducts(): Promise<Product[]> {
    return productsApi.listProducts(this.transport);
  }

  createProduct(input: CreateProductInput): Promise<Product> {
    return productsApi.createProduct(this.transport, input);
  }

  updateProduct(id: string, input: UpdateProductInput): Promise<Product> {
    return productsApi.updateProduct(this.transport, id, input);
  }

  listNutrients(): Promise<Nutrient[]> {
    return nutrientsApi.listNutrients(this.transport);
  }

  createNutrient(input: CreateNutrientInput): Promise<Nutrient> {
    return nutrientsApi.createNutrient(this.transport, input);
  }

  updateNutrient(id: string, input: UpdateNutrientInput): Promise<Nutrient> {
    return nutrientsApi.updateNutrient(this.transport, id, input);
  }

  listProductNutrients(productId: string): Promise<ProductNutrient[]> {
    return nutrientsApi.listProductNutrients(this.transport, productId);
  }

  replaceProductNutrients(
    productId: string,
    items: SaveProductNutrientInput[],
  ): Promise<ProductNutrient[]> {
    return nutrientsApi.replaceProductNutrients(this.transport, productId, items);
  }

  listInventoryLots(): Promise<InventoryLot[]> {
    return inventoryApi.listInventoryLots(this.transport);
  }

  createInventoryLot(input: CreateInventoryLotInput): Promise<InventoryLot> {
    return inventoryApi.createInventoryLot(this.transport, input);
  }

  updateInventoryLot(id: string, input: UpdateInventoryLotInput): Promise<InventoryLot> {
    return inventoryApi.updateInventoryLot(this.transport, id, input);
  }

  listStockMovements(): Promise<StockMovement[]> {
    return inventoryApi.listStockMovements(this.transport);
  }

  listStockMovementsForLot(lotId: string): Promise<StockMovement[]> {
    return inventoryApi.listStockMovementsForLot(this.transport, lotId);
  }

  listDoseLogs(input?: Partial<DoseLogListInput>): Promise<DoseLogPage> {
    return dosingApi.listDoseLogs(this.transport, input);
  }

  createDoseLog(input: CreateDoseLogInput): Promise<DoseLog> {
    return dosingApi.createDoseLog(this.transport, input);
  }

  updateDoseLog(id: string, input: UpdateDoseLogInput): Promise<DoseLog> {
    return dosingApi.updateDoseLog(this.transport, id, input);
  }

  cancelDoseLog(id: string): Promise<DoseLog> {
    return dosingApi.cancelDoseLog(this.transport, id);
  }

  listDoseLogNutrients(doseLogId: string): Promise<DoseLogNutrient[]> {
    return dosingApi.listDoseLogNutrients(this.transport, doseLogId);
  }

  listStorageLocations(includeArchived = false): Promise<StorageLocation[]> {
    return storageLocationsApi.listStorageLocations(this.transport, includeArchived);
  }

  createStorageLocation(input: CreateStorageLocationInput): Promise<StorageLocation> {
    return storageLocationsApi.createStorageLocation(this.transport, input);
  }

  updateStorageLocation(
    id: string,
    input: UpdateStorageLocationInput,
  ): Promise<StorageLocation> {
    return storageLocationsApi.updateStorageLocation(this.transport, id, input);
  }

  archiveStorageLocation(id: string): Promise<StorageLocation> {
    return storageLocationsApi.archiveStorageLocation(this.transport, id);
  }

  getDailyIntakeReport(date: string, timezoneOffsetMinutes: number): Promise<DailyIntakeReport> {
    return reportsApi.getDailyIntakeReport(this.transport, date, timezoneOffsetMinutes);
  }

  getNutrientTrendReport(
    input: NutrientTrendReportInput,
  ): Promise<NutrientTrendReport> {
    return reportsApi.getNutrientTrendReport(this.transport, input);
  }

  listBackups(): Promise<BackupList> {
    return adminApi.listBackups(this.transport);
  }

  createBackup(): Promise<BackupFile> {
    return adminApi.createBackup(this.transport);
  }

  getDatabaseOverview(): Promise<DatabaseOverview> {
    return adminApi.getDatabaseOverview(this.transport);
  }

  listDatabaseTables(): Promise<DatabaseTableStats[]> {
    return adminApi.listDatabaseTables(this.transport);
  }

  getRuntimeConfiguration(): Promise<RuntimeConfiguration> {
    return adminApi.getRuntimeConfiguration(this.transport);
  }

  validateProfile(profileId: string): Promise<ProfileValidation> {
    return adminApi.validateProfile(this.transport, profileId);
  }


}
