--Mindful Workforce
--By Aliya Smith

--Define functions

--Define a calculateHours function to calculate hours worked on a project
function calculateHours(timePeriods, hoursPerPeriod)
    local totalHours = 0
    for key, timePeriod in pairs(timePeriods) do
        totalHours = totalHours + hoursPerPeriod[key] * timePeriod
    end
    return totalHours
end

--Define a generateEmployeeSummary function to generate summary of employees on a project
function generateEmployeeSummary(employees, hours)
    employeeSummary = {}
    for key, employee in pairs(employees) do
        if hours[key] ~= nil then
            local hoursWorked = hours[key]
            employeeSummary[key] = {
                employee = employee,
                hoursWorked = hoursWorked
            }
        end
    end
    return employeeSummary
end

--Define a generateTimePeriodSummary function to generate summary of hours worked in each time period
function generateTimePeriodSummary(timePeriods, hoursPerPeriod)
    timePeriodSummary = {}
    for key, period in pairs(timePeriods) do
        local hours = period * hoursPerPeriod[key]
        timePeriodSummary[key] = {
            timePeriod = period,
            hours = hours
        }
    end
    return timePeriodSummary
end


--Define a printSummaryTable function to print summary tables to the screen
function printSummaryTable(summaryTable)
    print('Summary Table')
    for key, values in pairs(summaryTable) do
        print(key)
        for skey, svalue in pairs(values) do
            print('  ' .. skey .. ': ' .. svalue)
        end
    end
end


--Define main
function main()
    --Define project details
    local timePeriods = {
        'week1',
        'week2',
        'week3',
        'week4',
    }
    local hoursPerPeriod = {
        week1 = 40,
        week2 = 40,
        week3 = 40,
        week4 = 40,
    }
    local employees = {
        'John',
        'Sarah',
        'Matt',
    }
    local hours = {
        John = {
            week1 = 37,
            week2 = 37,
            week3 = 38,
            week4 = 40,
        },
        Sarah = {
            week1 = 40,
            week2 = 40,
            week3 = 40,
            week4 = 40,
        },
        Matt = {
            week1 = 40,
            week2 = 40,
            week3 = 37,
            week4 = 40,
        },
    }
    
    --Calculate total hours worked on the project
    local totalHours = calculateHours(timePeriods, hoursPerPeriod)
    
    --Generate employee summaries and time period summaries
    local employeeSummary = generateEmployeeSummary(employees, hours)
    local timePeriodSummary = generateTimePeriodSummary(timePeriods, hoursPerPeriod)
    
    --Print out summaries to the screen
    print('Total Hours Worked: ' .. totalHours .. '\n')
    printSummaryTable(employeeSummary)
    printSummaryTable(timePeriodSummary)
end

--Call main
main()