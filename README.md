# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder. 
If you chose to develop with the router feature, you will also have a `views` folder.

## Getting Started

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

## Style guidelines


### Naming Things

* *Add* to a list.
* *Remove* from a list.
* *Create* a new entity.
* *Archive*: Non-permanently hide or archive an entity.
* *Delete*: Permanently delete an entity.
* *Update* an entity or value.
* *View* an entity or value.
* *Change*: Pending changes to an entity.
* *New*: Pending entity to be created.

`[noun][verb][subverb]`

For example:

  ConsumableUpdateBasic
  ConsumableUpdateIngredients
