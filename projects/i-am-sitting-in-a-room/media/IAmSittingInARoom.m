classdef IAmSittingInARoom < audioPlugin
    properties
        Run = false;
        Save = false;
        Reset = false;
    end
    properties(Access = private)
        SampleRate
        source;
        srcfile = 'input.wav';
        Pause = 3; %sec
        delayed = true;
        StartDelay = 10; %sec
        srcDur = 0; %sec (0 for full)
        srcLength;
        bufplay;
        bufrec;
        i = 1;
        nccl = 1;
    end
    properties (Constant)
        PluginInterface = audioPluginInterface(...
            audioPluginParameter('Run',...
            'DisplayName','Run/pause',...
            'Mapping',{'enum', '0', '1'}), ...
            audioPluginParameter('Save',...
            'DisplayName','Save loops',...
            'Mapping',{'enum', '0', '1'}), ...
            audioPluginParameter('Reset',...
            'DisplayName','Reset',...
            'Mapping',{'enum', '0', '1'}));
    end
    methods
        
        function plg = IAmSittingInARoom
            plg.SampleRate = getSampleRate(plg);
            plg.source = audioread(plg.srcfile);
            if plg.srcDur==0
                plg.srcLength = size(plg.source,1);
            else
                plg.srcLength = plg.srcDur * plg.SampleRate;
                plg.source = plg.source(1:plg.srcLength,:);
            end
            plg.bufplay = [plg.source; ...
                zeros(plg.SampleRate*plg.Pause, 2)];
            plg.bufrec = zeros( plg.srcLength + plg.SampleRate*plg.Pause, 2);
        end
        
        function out = process(plg,in)
            frameSize = size(in,1);
            out = zeros(frameSize,2);
            if plg.Run
                if plg.delayed
                    plg.i = plg.i + frameSize;
                    if plg.i > plg.StartDelay * plg.SampleRate
                        plg.i = 1;
                        plg.delayed = false;
                    end
                else
                    out(1:min(frameSize, size(plg.bufplay,1)-plg.i+1), :) = ...
                        plg.bufplay(plg.i:min(plg.i+frameSize-1, size(plg.bufplay,1)), :);
                    
                    inrec = in(1:min(frameSize, size(plg.bufplay,1)-plg.i+1), :);
                    plg.bufrec(plg.i:min(plg.i+frameSize-1, size(plg.bufplay,1)), :) = ...
                        inrec;
                    %                     inrec .* std(out) ./ std(inrec);
                    
                    plg.i = plg.i + frameSize;
                    if plg.i>size(plg.bufplay,1)
                        plg.i=1;
                        plg.bufrec(:,2) = plg.bufrec(:,1);
                        plg.bufplay = plg.bufrec .* std(plg.bufplay) ./ std(plg.bufrec);
%                         plg.bufplay = plg.bufrec;
                        plg.nccl = plg.nccl + 1;
                        if plg.Save
                            audiowrite( sprintf('%s loop %i.wav', plg.srcfile(1:end-4), plg.nccl), ...
                                plg.bufplay, plg.SampleRate);
                        end
                    end
                    
                end
            end
            
            if plg.Reset
                plg.bufplay = [plg.source; ...
                    zeros(plg.SampleRate*plg.Pause, 2)];
                plg.bufrec = zeros( plg.srcLength + ...
                    plg.SampleRate*plg.Pause, 2);
                plg.i = 1;
                plg.nccl = 1;
                plg.delayed = true;
                plg.Run = false;
                plg.Save = false;
                plg.Reset = false;
            end
            
        end
        
        function reset(plugin)
            plugin.SampleRate = getSampleRate(plugin);
        end
        
    end
end