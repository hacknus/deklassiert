# deklassiert

_deklassiert_ is a tiny web project that lists all SBB IC8, IC81, IC6 or IC61 with 1. class coaches operating as 2. class coaches.
The project is built with [Dioxus](https://dioxuslabs.com/) and [Tailwind CSS](https://tailwindcss.com/).

First you need to download the SBB fahrplan icons from [here](https://digital.sbb.ch/de/foundation/assets/fpl/) and place the `sbb-icons-main` folder in the `asset` folder.
Add your API tokens from [https://api-manager.opentransportdata.swiss](https://api-manager.opentransportdata.swiss) in the `.env` file as described in the `.env.example` file.
Download the SBB fonts from [here](https://brand.sbb.ch/d/k7TF5BpX3B3F/markenelemente#/markenelemente/typografie)


### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

or build with docker compose:

```bash
docker compose up -d --build
```

