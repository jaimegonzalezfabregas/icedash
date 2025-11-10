const ffmpeg = require('fluent-ffmpeg');
const path = require('path');
const fs = require('fs');

const audio_chunk_names = [
    "move_1", "move_2", "move_3", "move_4", "move_5",
    "move_6", "move_7", "move_8", "move_9", "move_10",
    "move_11", "move_12", "move_13", "move_14", "move_15",
    "move_16", "move_17", "too_many_moves", "won_room",
    "won_strech", "hit_box", "change_room"
];
const chunkDuration = 4;

const input_audio_file = 'src.mp3';
const outputDir = "../";

const splitAudio = (inputFile, chunkNames) => {

    ffmpeg.ffprobe(inputFile, (err, metadata) => {
        if (err) {
            console.error("Error reading audio file: ", err);
            return;
        }


        for (let i = 0; i < chunkNames.length; i++) {
            const start = i * chunkDuration;
            console.log("start of chunk is", start, "duration is", chunkDuration);
            const outputFileName = path.join(outputDir, `${chunkNames[i]}.mp3`);

            ffmpeg(inputFile)
                .setStartTime(start)
                .setDuration(chunkDuration)
                .output(outputFileName)
                .on('end', () => {
                    console.log(`Created ${outputFileName}`);
                })
                .on('error', (err) => {
                    console.error(`Error processing chunk: ${err.message}`);
                })
                .run();
        }
    });
};

splitAudio(input_audio_file, audio_chunk_names);