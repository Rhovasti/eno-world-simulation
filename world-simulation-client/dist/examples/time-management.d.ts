/**
 * Time Management Example for World Simulation Client
 * Demonstrates time queries, calculations, and valley time zones
 */
declare function timeManagementExample(): Promise<void>;
export declare function getTimeContext(): Promise<{
    current_hour: any;
    formatted_date: string;
    auto_tick_enabled: any;
    tick_interval_ms: any;
    valley_time_zones: import("../index.js").ValleyTimeZone[];
    simulation_status: any;
}>;
export { timeManagementExample, getTimeContext };
//# sourceMappingURL=time-management.d.ts.map