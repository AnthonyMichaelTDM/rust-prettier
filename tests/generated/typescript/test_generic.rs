use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_return_type_ts_format_1_fa44743a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<Descriptor> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<Descriptor | undefined> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssembly[\"attributes\"] | undefined\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<Descriptor & undefined> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssembly[\"attributes\"] & undefined\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<Descriptor[\"attributes\"]> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssembly[\"attributessssssssssssssssssssssss\"]\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<keyof Descriptor> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<\n  keyof Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<Descriptor[]> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy[]\n> => {};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<Descriptor> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<Descriptor | undefined> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssembly[\"attributes\"] | undefined\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<Descriptor & undefined> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssembly[\"attributes\"] & undefined\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<Descriptor[\"attributes\"]> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssembly[\"attributessssssssssssssssssssssss\"]\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<keyof Descriptor> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<\n  keyof Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy\n> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<Descriptor[]> => {};\n\nexport const getVehicleDescriptor = async (\n  vehicleId: string,\n): Promise<\n  Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy[]\n> => {};");
}
#[test]
fn test_issue_6899_ts_format_1_cb72c001() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const getUnusedAuthorizationHoldDocuments = async (): Promise<DocumentData[]> => {}\n\nconst firestorePersonallyIdentifiablePaths: Array<\n  keyof Collections.Users.Entity\n> = []\n\nexport const SUPPORTED_VEHICLE_TYPES: Array<\n  Collections.VehiclesStates.Entity['type']\n> = Object.values(Collections.VehiclesStates.Type);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const getUnusedAuthorizationHoldDocuments = async (): Promise<\n  DocumentData[]\n> => {};\n\nconst firestorePersonallyIdentifiablePaths: Array<\n  keyof Collections.Users.Entity\n> = [];\n\nexport const SUPPORTED_VEHICLE_TYPES: Array<\n  Collections.VehiclesStates.Entity[\"type\"]\n> = Object.values(Collections.VehiclesStates.Type);");
}
#[test]
fn test_object_method_ts_format_1_b4a911f9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("export default {\n  load<K, T>(k: K, t: T) {\n    return {k, t};\n  }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "export default {\n  load<K, T>(k: K, t: T) {\n    return { k, t };\n  },\n};"
    );
}
#[test]
fn test_ungrouped_parameters_ts_format_1_a3623372() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function filterTooltipWithFoo<F extends Field>(oldEncoding: Encoding<F>): {\n  customTooltipWithoutAggregatedField?:\n    | StringFieldDefWithCondition<F>\n    | StringValueDefWithCondition<F>\n    | StringFieldDef<F>[];\n  filteredEncoding: Encoding<F>;\n} {\n  const {tooltip, ...filteredEncoding} = oldEncoding;\n  if (!tooltip) {\n    return {filteredEncoding};\n  }\n  // ...\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function filterTooltipWithFoo<F extends Field>(\n  oldEncoding: Encoding<F>,\n): {\n  customTooltipWithoutAggregatedField?:\n    | StringFieldDefWithCondition<F>\n    | StringValueDefWithCondition<F>\n    | StringFieldDef<F>[];\n  filteredEncoding: Encoding<F>;\n} {\n  const { tooltip, ...filteredEncoding } = oldEncoding;\n  if (!tooltip) {\n    return { filteredEncoding };\n  }\n  // ...\n}");
}
