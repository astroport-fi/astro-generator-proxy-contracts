use crate::errors::ContractError;
use crate::states::Config;
use cosmwasm_std::Uint128;

pub fn register_schedules(
    mut current_block: u64,
    config: &mut Config,
    schedules: Vec<(u64, u64, Uint128)>,
    amount: Uint128,
) -> Result<(), ContractError> {
    let mut to_deposit = Uint128::zero();

    for schedule in schedules {
        to_deposit = to_deposit.checked_add(schedule.2)?;

        if !(schedule.0 >= current_block && schedule.0 < schedule.1 && schedule.1 > current_block) {
            return Err(ContractError::ScheduleError {});
        }

        config.distribution_schedule.push(schedule);
        // start block of next schedule should be higher then end block of previous schedule
        current_block = schedule.1;
    }

    if to_deposit != amount {
        return Err(ContractError::SchedulesAmountError {});
    }

    Ok(())
}
