% create moon program entry point
                    entry
                    addi r14,r0,topaddr
                    hlt

% space for variable sample
sample              res 400
% space for variable idx
idx                 res 4
% space for variable maxValue
maxValue            res 4
% space for variable minValue
minValue            res 4
% space for variable utility
utility             res 40328
% space for variable arrayUtility
arrayUtility        res 10162656
