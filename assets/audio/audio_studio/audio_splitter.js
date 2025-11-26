const { exec } = require('child_process');
const path = require('path');
const fs = require("fs");

const audio_chunk_names = [
    "move_1", "move_2", "move_3", "move_4", "move_5",
    "move_6", "move_7", "move_8", "move_9", "move_10",
    "move_11", "move_12", "move_13", "move_14", "move_15",
    "move_16", "move_17", "too_many_moves", "won_room",
    "won_strech", "hit_box", "change_room", "hit_weak_wall", "start_strech"
];

const chunkDuration = 4; // Duration of each chunk in seconds
const input_audio_file = 'src.mp3';
const outputDir = "../";


// Function to remove all .mp3 files in the output directory
const removeMp3Files = (directory) => {
    fs.readdirSync(directory, (err, files) => {
        if (err) {
            console.error(`Unable to read directory: ${err}`);
            return;
        }

        files.forEach(file => {
            if (path.extname(file) === '.mp3') {
                const filePath = path.join(directory, file);
                fs.unlinkSync(filePath, (err) => {
                    if (err) {
                        console.error(`Error deleting file ${file}: ${err}`);
                    } else {
                        console.log(`Deleted ${file}`);
                    }
                });
            }
        });
    });
};

const getMeanVolume = (inputFile) => {
    return new Promise((resolve, reject) => {
        const command = `ffmpeg -i ${inputFile} -af "volumedetect" -f null /dev/null`;

        exec(command, (err, stdout, stderr) => {
            if (err) {
                console.error(`Error detecting volume for ${inputFile}: ${stderr}`);
                return reject(err);
            }

            const meanMatch = stderr.match(/mean_volume:\s*(-*\d+(\.\d+)?)/);
            if (meanMatch) {
                const meanVolume = parseFloat(meanMatch[1]);
                resolve(meanVolume);
            } else {
                reject(new Error("Could not determine mean volume."));
            }
        });
    });
};

const removeSilence = async (inputFile, outputFile) => {
    try {
        const meanVolume = await getMeanVolume(inputFile);
        console.log(meanVolume);
        const command = `ffmpeg -i ${inputFile} -af "silenceremove=stop_periods=-1:stop_duration=0.1:stop_threshold=${meanVolume-30}dB" -y ${outputFile}`;

        exec(command, (err, stdout, stderr) => {
            // console.log("remove silence", stderr, stdout);
            if (err) {
                console.error(`Error processing ${inputFile}: ${stderr}`);
                return;
            }
            console.log(`Removed silence from ${inputFile}, saved as ${outputFile}`);
        });
    } catch (error) {
        console.error(error);
    }
};


const splitAudio = async (inputFile, chunkNames) => {
    for (let i = 0; i < chunkNames.length; i++) {
        const start = i * chunkDuration;
        const fullOutputFileName = path.join(outputDir, `full_${chunkNames[i]}.mp3`);

        // Command to split the audio
        const splitCommand = `ffmpeg -i ${inputFile} -ss ${start} -t ${chunkDuration} -y ${fullOutputFileName}`;

        exec(splitCommand, async (err, stdout, stderr) => {
            if (err) {
                console.error(`Error processing chunk ${chunkNames[i]}: ${stderr}`);
                return;
            }
            console.log(`Created ${fullOutputFileName}`);

            // Remove silence from the chunk after creation
            const cleanedOutputName = path.join(outputDir, `${chunkNames[i]}.mp3`);
            await removeSilence(fullOutputFileName, cleanedOutputName);
        });
    }
};

removeMp3Files(outputDir);

// Start the process
splitAudio(input_audio_file, audio_chunk_names).catch(err => console.error(err));
