# velho

Velho (wizard in Finnish) is a desktop application developed using Tauri and React. 
It provides a user interface for controlling Philips Wiz smart lights on the same network as your device is on.

## Features
- [x] Discover lights on the network automatically
- [x] Toggle light state
- [x] Change light temperature
- [x] Change light dimming
- [x] Name lights (by default the name is the light's MAC address)
- [ ] Change light color 
- [ ] Create/edit/toggle custom scenes for lights
- [ ] Create/edit custom groups of lights
- [ ] Control groups of lights

## Screenshots
![](https://raw.githubusercontent.com/natixco/velho/main/screenshots/lights.png)
![](https://raw.githubusercontent.com/natixco/velho/main/screenshots/light.png)

## Installation
### Prerequisites
- Node.js and npm installed
- Rust and Cargo installed

### Steps
1. Clone the repository
1. Navigate to the project directory
1. Run `npm install` to install the necessary dependencies
1. Run `npm run tauri-dev` to start the development server and the application

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)
