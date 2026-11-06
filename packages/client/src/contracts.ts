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
  DoseLog,
  DoseLogListInput,
  DoseLogNutrient,
  DoseLogPage,
  InventoryLot,
  Nutrient,
  NutrientTrendReport,
  Product,
  ProductNutrient,
  ProfileValidation,
  RuntimeConfiguration,
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



export type NutrientTrendReportInput = {
  startDate: string;
  endDate: string;
  timezoneOffsetMinutes: number;
  dayStartHour?: number;
  nutrientId?: string | null;
  productId?: string | null;
  unit?: string | null;
};

/** Transport-neutral operations used by DoseArk feature code. */
export interface DoseArkClient {
  health(timeoutMs?: number, healthPath?: string): Promise<SystemHealth>;

  listBrands(): Promise<Brand[]>;
  createBrand(input: CreateBrandInput): Promise<Brand>;
  updateBrand(id: string, input: UpdateBrandInput): Promise<Brand>;

  listProducts(): Promise<Product[]>;
  createProduct(input: CreateProductInput): Promise<Product>;
  updateProduct(id: string, input: UpdateProductInput): Promise<Product>;

  listNutrients(): Promise<Nutrient[]>;
  createNutrient(input: CreateNutrientInput): Promise<Nutrient>;
  updateNutrient(id: string, input: UpdateNutrientInput): Promise<Nutrient>;
  listProductNutrients(productId: string): Promise<ProductNutrient[]>;
  replaceProductNutrients(productId: string, items: SaveProductNutrientInput[]): Promise<ProductNutrient[]>;

  listInventoryLots(): Promise<InventoryLot[]>;
  createInventoryLot(input: CreateInventoryLotInput): Promise<InventoryLot>;
  updateInventoryLot(id: string, input: UpdateInventoryLotInput): Promise<InventoryLot>;
  listStockMovements(): Promise<StockMovement[]>;
  listStockMovementsForLot(lotId: string): Promise<StockMovement[]>;

  listDoseLogs(input?: Partial<DoseLogListInput>): Promise<DoseLogPage>;
  createDoseLog(input: CreateDoseLogInput): Promise<DoseLog>;
  updateDoseLog(id: string, input: UpdateDoseLogInput): Promise<DoseLog>;
  cancelDoseLog(id: string): Promise<DoseLog>;
  listDoseLogNutrients(doseLogId: string): Promise<DoseLogNutrient[]>;

  listStorageLocations(includeArchived?: boolean): Promise<StorageLocation[]>;
  createStorageLocation(input: CreateStorageLocationInput): Promise<StorageLocation>;
  updateStorageLocation(id: string, input: UpdateStorageLocationInput): Promise<StorageLocation>;
  archiveStorageLocation(id: string): Promise<StorageLocation>;

  getDailyIntakeReport(date: string, timezoneOffsetMinutes: number): Promise<DailyIntakeReport>;
  getNutrientTrendReport(input: NutrientTrendReportInput): Promise<NutrientTrendReport>;

  listBackups(): Promise<BackupList>;
  createBackup(): Promise<BackupFile>;
  getDatabaseOverview(): Promise<DatabaseOverview>;
  listDatabaseTables(): Promise<DatabaseTableStats[]>;
  getRuntimeConfiguration(): Promise<RuntimeConfiguration>;
  validateProfile(profileId: string): Promise<ProfileValidation>;
}

/** HTTP-specific extension used by remote runtime profiles. */
export interface DoseArkRemoteClient extends DoseArkClient {
  setBaseUrl(baseUrl: string): void;
  getBaseUrl(): string;
}
