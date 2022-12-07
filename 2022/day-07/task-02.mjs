import readline from 'node:readline';

const rl = readline.createInterface({ input: process.stdin });
const path = [];
const directories = { '/': { content: [], subdirectories: [] } };

function buildPath(...elements) {
    return elements.join('/').slice(1) || '/';
}

for await (const line of rl) {
  if (line.startsWith("$ cd ")) {
    let dir = line.slice(5).trim();

    if (dir === '/') {
        path.length = 0;
        path.push(dir);
    } else if (dir === '..') {
        path.pop();
    } else {
        path.push(dir);
        directories[buildPath(...path)] ||= { content: [], subdirectories: [] };
    }
  } else if (line === '$ ls') {
    // Do nothing because the lines which follow are handled by branches below.
  } else if (line.startsWith('dir')) {
    directories[buildPath(...path)].subdirectories.push(buildPath(...path, line.slice(4)));
  } else { // file
    const [size, name] = line.split(' ');
    directories[buildPath(...path)].content.push([parseInt(size, 10), name]);
  }
}

const numDirectories = Object.keys(directories).length;
const calculated = [];

while (calculated.length < numDirectories) {
    for (const [dir, info] of Object.entries(directories)) {
        if (typeof info.size === 'undefined' && info.subdirectories.every(dir => calculated.includes(dir))) {
            const contentSum = info.content.reduce((total, entry) => total + entry[0], 0);
            const subdirSum = info.subdirectories.reduce((total, dir) => total + directories[dir].size, 0);
            info.size = contentSum + subdirSum;
            calculated.push(dir);
        }
    }
}

const sizes = Object.values(directories).map(info => info.size);
sizes.sort((a, b) => a - b);

const remaining = 70000000 - directories['/'].size;
const needed = 30000000 - remaining;

for (const size of sizes) {
    if (size > needed) {
        console.log(size);
        break;
    }
}
