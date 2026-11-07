//! Root imports, legacy system imports and feature module paths remain identical.

use std::any::TypeId;

/// Comparing TypeId also confirms that re-exports are aliases, not new DTOs.
fn same_type<T: 'static, U: 'static>() {
    assert_eq!(TypeId::of::<T>(), TypeId::of::<U>());
}

#[test]
fn root_and_system_reexports_are_compatible() {
    same_type::<doseark_contracts::HealthResponse, doseark_contracts::system::HealthResponse>();
    same_type::<
        doseark_contracts::DatabaseHealthResponse,
        doseark_contracts::system::health::DatabaseHealthResponse,
    >();
    same_type::<
        doseark_contracts::StorageCapabilitiesResponse,
        doseark_contracts::system::database::StorageCapabilitiesResponse,
    >();
    same_type::<
        doseark_contracts::DatabaseOverviewResponse,
        doseark_contracts::system::database::DatabaseOverviewResponse,
    >();
    same_type::<
        doseark_contracts::DatabaseTableStatsResponse,
        doseark_contracts::system::DatabaseTableStatsResponse,
    >();
    same_type::<
        doseark_contracts::RuntimeConfigValueResponse,
        doseark_contracts::system::runtime::RuntimeConfigValueResponse,
    >();
    same_type::<
        doseark_contracts::RuntimeEnvironmentVariableResponse,
        doseark_contracts::system::RuntimeEnvironmentVariableResponse,
    >();
    same_type::<
        doseark_contracts::RuntimeConfigurationResponse,
        doseark_contracts::system::runtime::RuntimeConfigurationResponse,
    >();
    same_type::<
        doseark_contracts::ProfileValidationCheckResponse,
        doseark_contracts::system::profiles::ProfileValidationCheckResponse,
    >();
    same_type::<
        doseark_contracts::ProfileValidationResponse,
        doseark_contracts::system::ProfileValidationResponse,
    >();
}
