-- Automation script for performing 'I Am Sitting in a Room' by Alvin Lucier
-- How to use:
-- open blank project and create two tracks: for recording and playback
-- open this script with Reaper's script interface (https://www.reaper.fm/sdk/reascript/reascript.php)
-- record something into the first track or paste 'seed audio'
-- run the script
function CopyRecordingToPlayback(item, destTrack, position)
    reaper.PreventUIRefresh(1)

    local retval, chunk = reaper.GetItemStateChunk(item, "", false)

    local copied_item = reaper.AddMediaItemToTrack(destTrack)
    reaper.SetItemStateChunk(copied_item, chunk, false)
    reaper.SetMediaItemInfo_Value(copied_item, "D_POSITION", position)
    -- set nice fadein/out for smooth playback
    reaper.SetMediaItemInfo_Value(copied_item, "D_FADEINLEN", 0.5)
    reaper.SetMediaItemInfo_Value(copied_item, "D_FADEOUTLEN", 0.5)
    local take = reaper.GetMediaItemTake(copied_item, 0)
    -- mixdown take to mono
    reaper.SetMediaItemTakeInfo_Value(take, "I_CHANMODE", 2)
    -- handle normalization
    local source = reaper.GetMediaItemTake_Source(take)
    local gain =
        reaper.CalculateNormalization(source, 0, -- 0=LUFS-I, 1=RMS-I, 2=peak, 3=true peak, 4=LUFS-M max, 5=LUFS-S max
        -12, -- normalizeTarget
        0, -- normalizeStart
        0 -- normalizeEnd, 0 for full
        )
    reaper.SetMediaItemInfo_Value(copied_item, "D_VOL", gain)
    reaper.PreventUIRefresh(-1)
    return copied_item
end

-- setup
reaper.ClearAllRecArmed()
reaper.MuteAllTracks(1)
reaper.GetSetRepeat(0)
reaper.GetSet_LoopTimeRange(false, false, 0, 0, false)

-- configure track states
rec_track = reaper.GetTrack(0, 0)
playback_track = reaper.GetTrack(0, 1)
reaper.SetTrackUIMute(playback_track, 0, 1)
reaper.SetTrackUIRecArm(rec_track, 1, 1)
reaper.SetTrackUIInputMonitor(rec_track, 0, 1)

-- main async loop

DURATION_INCREMENT = 0.5 -- each iteration adds this many seconds
rec_item_idx = 0 -- pre-recorded seed item idx
next_iteration_at = 0
first_run = true

function iteration()
    if os.time() < next_iteration_at then
        reaper.defer(iteration) -- schedule next iteration
        return
    end

    -- if user paused, abort the script
    if not first_run and reaper.GetPlayState() == 0 then
        reaper.ShowConsoleMsg("user paused, aborting")
        return
    end

    reaper.ShowConsoleMsg("iteration...\n")
    first_run = false

    -- stop recording
    reaper.Main_OnCommand(40667, 0)

    prev_rec = reaper.GetTrackMediaItem(rec_track, rec_item_idx)
    rec_item_idx = rec_item_idx + 1
    prev_rec_start = reaper.GetMediaItemInfo_Value(prev_rec, "D_POSITION")
    prev_rec_duration = reaper.GetMediaItemInfo_Value(prev_rec, "D_LENGTH")
    playback_start = prev_rec_start + prev_rec_duration + 5
    rec_start = playback_start - DURATION_INCREMENT / 2
    rec_duration = prev_rec_duration + DURATION_INCREMENT
    CopyRecordingToPlayback(prev_rec, playback_track, playback_start)
    -- set edit cursor
    reaper.ApplyNudge(0, 1, 6, 1, rec_start, false, 0)
    -- start recording
    reaper.Main_OnCommand(1013, 0)
    -- wait for the playback
    next_iteration_at = os.time() + rec_duration
    reaper.defer(iteration)
end

reaper.defer(iteration)
