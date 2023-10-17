/*!
 * 
 * This file is an extension for the Lavendeux parser
 * It is a text-based adventure game playable in:
 * - Lavendeux
 * - Web browser
 * - node.js
 * 
 * https://rscarson.github.io/lavendeux/
 * 
 */
const name = "zarbans_grotto";
const version = "1.0.0";
const author = "@rscarson";
class PlayerChoices {
  constructor(json = {}) {
    Object.assign(this, JSON.parse(JSON.stringify(
      json
    )));
    for (const i in this.records) {
      this.records[i] = new PlayerChoice(this.records[i]);
    }
  }
  /**
   * List the available entries
   * @returns Array
   */
  list() {
    return Object.keys(this.records);
  }
  /**
   * List the available entries
   * @returns Array
   */
  list_chosen() {
    return Object.keys(this.records).filter((k) => this.chose(k));
  }
  /**
   * Make a choice
   * @param {String} choice 
   */
  choose(choice, enabled) {
    this.records[choice].enabled = enabled;
  }
  /**
   * Returns true if the player chose the given path
   * @param {String} choice 
   * @returns boolean
   */
  chose(choice) {
    return this.records[choice].enabled;
  }
}
class PlayerChoice {
  constructor(json = {}) {
    Object.apply(this, json);
    if (this.enabled == void 0) {
      this.enabled = false;
    }
  }
}
class PlayerEffect {
  constructor(json = {}) {
    Object.assign(this, json);
  }
  static build(json = {}) {
    switch (json.type) {
      case "status":
        return new PlayerStatusEffect(json);
      case "choices":
        return new PlayerChoiceEffect(json);
      case "inventory":
        return new PlayerInventoryEffect(json);
    }
  }
  /**
   * Verify this condition
   * @param {object} playerStatus 
   * @returns boolean
   */
  verify(player) {
    if (this.target == "all") {
      return player[this.type].list().map((e) => this.verifyTarget(e, player[this.type])).filter((e) => e == false).length == 0;
    } else {
      return this.verifyTarget(this.target, player[this.type]);
    }
  }
  /**
   * Apply this effect
   * @param {object} playerStatus 
   */
  apply(player, skip_inv_queue = false) {
    if (this.target == "all") {
      player[this.type].list().map((e) => this.applyToTarget(e, player[this.type])).filter((e) => e == false).length == 0;
    } else {
      this.applyToTarget(this.target, player[this.type]);
    }
    if (!skip_inv_queue) {
      for (const e of player.inventory.addEffectQueue) {
        e.apply(player, true);
      }
      for (const e of player.inventory.delEffectQueue) {
        e.remove(player);
      }
      player.inventory.addEffectQueue = [];
      player.inventory.delEffectQueue = [];
    }
  }
  /**
   * Remove this effect
   * @param {object} playerStatus 
   */
  remove(player) {
    if (this.target == "all") {
      return player[this.type].list().map((e) => this.removeFromTarget(e, player[this.type])).filter((e) => e == false).length == 0;
    } else {
      this.removeFromTarget(this.target, player[this.type]);
    }
  }
}
class PlayerStatusEffect extends PlayerEffect {
  /**
   * Verify this condition against a specific target
   * @param {String} target 
   * @param {object} data 
   * @returns boolean
   */
  verifyTarget(target, data) {
    switch (this.operation) {
      case "lt":
        return data.get(target).value < this.value;
      case "lte":
        return data.get(target).value <= this.value;
      case "gt":
        return data.get(target).value > this.value;
      case "gte":
        return data.get(target).value >= this.value;
      case "eq":
        return data.get(target).value == this.value;
      case "ne":
        return data.get(target).value != this.value;
      default:
        throw new Error("Invalid operation for statusEffect");
    }
  }
  /**
   * Apply this effect to a specific target
   * @param {String} target 
   * @param {object} data 
   */
  applyToTarget(target, data) {
    switch (this.operation) {
      case "add":
        return data.get(target).add(this.value);
      case "add_max":
        return data.get(target).maximum += this.value;
    }
  }
  /**
   * Remove this effect from a specific target
   * @param {String} target 
   * @param {object} data 
   */
  removeFromTarget(target, data) {
    switch (this.operation) {
      case "add":
        return data.get(target).add(-this.value);
      case "add_max":
        return data.get(target).maximum -= this.value;
    }
  }
}
class PlayerChoiceEffect extends PlayerEffect {
  /**
   * Verify this condition against a specific target
   * @param {String} target 
   * @param {object} data 
   * @returns boolean
   */
  verifyTarget(target, data) {
    return this.value == data.chose(target);
  }
  /**
   * Apply this effect to a specific target
   * @param {String} target 
   * @param {object} data 
   */
  applyToTarget(target, data) {
    data.choose(target, this.value);
  }
  /**
   * Remove this effect from a specific target
   * @param {String} target 
   * @param {object} data 
   */
  removeFromTarget(target, data) {
    data.choose(target, !this.value);
  }
}
class PlayerInventoryEffect extends PlayerEffect {
  /**
   * Verify this condition against a specific target
   * @param {String} target 
   * @param {object} data 
   * @returns boolean
   */
  verifyTarget(target, data) {
    return data.has(target) == this.value;
  }
  /**
   * Apply this effect to a specific target
   * @param {String} target 
   * @param {object} data 
   */
  applyToTarget(target, data) {
    this.value ? data.give(target) : data.take(target);
  }
  /**
   * Remove this effect from a specific target
   * @param {String} target 
   * @param {object} data 
   */
  removeFromTarget(target, data) {
    this.value ? data.take(target) : data.give(target);
  }
}
class PlayerInventory {
  constructor(json = {}) {
    Object.assign(this, JSON.parse(JSON.stringify(
      json
    )));
    for (const i in this.records) {
      this.records[i] = new PlayerInventoryItem(this.records[i]);
    }
    this.addEffectQueue = [];
    this.delEffectQueue = [];
  }
  /**
   * List the available entries
   * @returns Array
   */
  list() {
    return Object.keys(this.records);
  }
  /**
   * List the available entries
   * @returns Array
   */
  list_equipped() {
    return Object.keys(this.records).filter((i) => this.has(i));
  }
  /**
   * List the equipped entries
   * @returns Array
   */
  all_equipped() {
    return Object.values(this.records).filter((i) => i.equipped);
  }
  /**
   * Set an item status
   * @param {String} item 
   * @param {Boolean} equipped 
   */
  set(item, equipped) {
    this.records[item].equipped = equipped;
  }
  /**
   * Give an item to the player
   * @param {String} item 
   * @returns array of effects to apply
   */
  give(item) {
    if (!this.has(item)) {
      this.addEffectQueue.push(...this.describe(item).effects);
    }
    this.set(item, true);
  }
  /**
   * Take an item away from the player
   * @param {String} item 
   */
  take(item) {
    if (this.has(item)) {
      this.delEffectQueue.push(...this.describe(item).effects);
    }
    this.set(item, false);
  }
  /**
   * Returns true if the player possesses the given item
   * @param {String} item 
   * @returns boolean
   */
  has(item) {
    return this.records[item].equipped;
  }
  /**
   * Returns item description
   * @param {String} item 
   * @returns boolean
   */
  describe(item) {
    return this.records[item];
  }
  /**
   * Return the full set of active effects from equipped items
   * @returns PlayEffect[]
   */
  activeEffects() {
    return Object.values(this.records).filter((i) => i.equipped).map((i) => i.effects).flat();
  }
}
class PlayerInventoryItem {
  constructor(json = {}) {
    Object.assign(this, json);
    for (const i in this.effects) {
      this.effects[i] = PlayerEffect.build(this.effects[i]);
    }
  }
}
class PlayerStatus {
  constructor(json = {}) {
    Object.assign(this, JSON.parse(JSON.stringify(
      json
    )));
    for (const i in this.records) {
      this.records[i] = new PlayerStatusItem(this.records[i]);
    }
  }
  /**
   * List the available status entries
   * @returns Array
   */
  list() {
    return Object.keys(this.records);
  }
  /**
   * List the available status entries
   * @returns Array
   */
  list_visible() {
    return this.list().filter((k) => !this.records[k].hidden);
  }
  /**
   * Retrieve a status
   * @param {String} target 
   * @returns Number
   */
  get(target) {
    return this.records[target];
  }
  /**
   * Retrieve a status value
   * @param {String} target 
   * @returns Number
   */
  value(target) {
    return this.records[target].value;
  }
  /**
   * Change a status value
   * @param {String} target 
   * @param {Number} value 
   */
  set(target, value) {
    this.get(target).set(value);
  }
  /**
   * Add to a status value
   * @param {String} target 
   * @param {Number} value 
   */
  add(target, value) {
    this.get(target).add(value);
  }
}
class PlayerStatusItem {
  constructor(json = {}) {
    Object.assign(this, json);
    if (this.value == void 0) {
      this.value = this.default;
    }
  }
  /**
   * Change a status value
   * @param {Number} value 
   */
  set(value) {
    this.value = value;
    if (this.value < 0) {
      this.value = 0;
    } else if (this.value > this.maximum) {
      this.value = this.maximum;
    }
  }
  /**
   * Add to a status value
   * @param {Number} value 
   */
  add(value) {
    this.set(this.value + value);
  }
}
class Chapter {
  constructor(json = {}) {
    Object.assign(this, JSON.parse(JSON.stringify(
      json
    )));
    for (const i in this.records) {
      this.records[i] = new Story(this.records[i]);
    }
  }
  /**
   * Retrieve a story in the chapter by ID
   * @param {String} story_key 
   * @returns A Story object, or false
   */
  getStory(story_key) {
    return this.records[story_key] || false;
  }
}
class Story {
  constructor(json = {}) {
    Object.assign(this, json);
    for (const i in this.effects) {
      this.effects[i] = PlayerEffect.build(this.effects[i]);
    }
    for (const i in this.options) {
      this.options[i] = new StoryOption(this.options[i]);
    }
  }
}
class StoryOption {
  constructor(json = {}) {
    Object.assign(this, json);
    for (const i in this.results) {
      this.results[i] = new StoryOptionResult(this.results[i]);
    }
    for (const i in this.conditions) {
      this.conditions[i] = PlayerEffect.build(this.conditions[i]);
    }
  }
  toString() {
    return this.prompt;
  }
}
class StoryOptionResult {
  constructor(json = {}) {
    if (typeof json === "string" || json instanceof String) {
      this.target = json;
      this.conditions = [];
    } else {
      Object.assign(this, json);
      for (const i in this.conditions) {
        this.conditions[i] = PlayerEffect.build(this.conditions[i]);
      }
    }
  }
}
class JsonUtilities {
  /**
   * Assign a copy of the source Object to target
   * @param {Object} target 
   * @param {Object} source 
   */
  static assign(target, source) {
    Object.assign(target, JSON.parse(JSON.stringify(
      source
    )));
  }
  static base64Encode(data) {
    if (typeof window.btoa === "function") {
      return window.btoa(data);
    } else if (typeof Buffer === "object") {
      return Buffer.from(data).toString("base64");
    } else {
      throw new Error("Cannot base64");
    }
  }
  static base64Decode(data) {
    if (typeof window.atob === "function") {
      return window.atob(data);
    } else if (typeof Buffer === "object") {
      return Buffer.from(data, "base64").toString("binary");
    } else {
      throw new Error("Cannot base64");
    }
  }
}
const $schema = "../schema/player.zarban.schema.json";
const entrypoint = "intro_cave1";
const chapters = [
  {
    $schema: "../schema/chapter.zarban.schema.json",
    name: "Chapter 1: The Grotto",
    records: {
      intro_cave1: {
        text: [
          "Drenched from the rain, and exhausted from having hunted all through the night, you have finally cornered the beast which you have been hired to dispatch.",
          "You approach the forboding cavern to which you have stalked your prey, the shapeshifting vampire Zarban.",
          "The foul stench of magic fills your nostrils as you prepare to enter the grotto proper."
        ],
        effects: [
          {
            type: "choices",
            target: "all",
            value: false
          },
          {
            type: "inventory",
            target: "all",
            value: false
          },
          {
            type: "status",
            target: "all",
            operation: "add",
            value: -99
          },
          {
            type: "status",
            target: "stamina",
            operation: "add",
            value: 2
          },
          {
            type: "inventory",
            target: "hunter_sword",
            value: true
          },
          {
            type: "inventory",
            target: "hunter_armor",
            value: true
          }
        ],
        options: [
          {
            prompt: "Enter the grotto",
            conditions: [],
            results: [
              "intro_cave2"
            ]
          }
        ]
      },
      intro_cave2: {
        text: [
          "In the darkness before you, deep within the cave looms a vile shadow, dripping with evil.",
          "It can only be the mighty vampire Zarban himself."
        ],
        effects: [],
        options: [
          {
            prompt: "Draw your magic sword and approach the shadow",
            conditions: [],
            results: [
              "intro_cave3_brave"
            ]
          },
          {
            prompt: "I don't care about vampires, let's go to the tavern",
            conditions: [],
            results: [
              "intro_cave3_tavern"
            ]
          }
        ]
      },
      intro_cave3_brave: {
        text: [
          "As you prepare yourself and draw your enchanted blade, you are knocked out from behind by a large rock to the head.",
          "You collapse to the ground, unconcious as the mighty Zarban scurries away into the night, cackling annoyingly."
        ],
        effects: [],
        options: [
          {
            prompt: "...",
            conditions: [],
            results: [
              "intro_cave4"
            ]
          }
        ]
      },
      intro_cave3_tavern: {
        text: [
          "As you turn around to give up your promising career as a mediocre vampire-hunter for hire, you are knocked out from behind by a large rock to the head.",
          "You collapse to the ground, unconcious as the mighty Zarban scurries away into the night, cackling annoyingly."
        ],
        effects: [],
        options: [
          {
            prompt: "...",
            conditions: [],
            results: [
              "intro_cave4"
            ]
          }
        ]
      },
      intro_cave4: {
        text: [
          "You awaken some time later, to find your magic blade, and mint-condition vintage vampire hunting armor reduced to worthless scrap before you.",
          "You gather what little you can salvage, and turn to leave the grotto.",
          "",
          "Before you lies a single set of footprints, leading away from the grotto."
        ],
        effects: [
          {
            type: "inventory",
            target: "all",
            value: false
          }
        ],
        options: [
          {
            prompt: "Follow the footprints",
            conditions: [],
            results: [
              "intro_cave5_brave"
            ]
          },
          {
            prompt: "I don't care about footprints, let's go to the tavern",
            conditions: [],
            results: [
              "intro_cave5_tavern"
            ]
          }
        ]
      },
      intro_cave5_brave: {
        text: [
          "You follow the footprints to a nearby village, and arrive just as dawn breaks.",
          "The footprints lead into the village, but too many footprints coming and going make it impossible to tell what happened next.",
          "",
          "One thing you can be sure of, however; you still sense Zarban's evil aura - you are sure he is still hiding out somewhere in this very town",
          "Likely having replaced one of the sleepy village's unsuspecting peasants.",
          "You are exausted from searching through the night, and could use a pick-me-up."
        ],
        effects: [],
        options: [
          {
            prompt: "Look around",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      intro_cave5_tavern: {
        text: [
          "You walk to a tavern in a nearby village, and arrive just as dawn breaks.",
          "",
          "As you approach the village, you once again sense Zarban's evil aura - you are sure he is hiding out somewhere in this very town",
          "Likely having replaced one of the sleepy village's unsuspecting peasants."
        ],
        effects: [],
        options: [
          {
            prompt: "Enter the tavern",
            conditions: [],
            results: [
              "tavern_enter"
            ]
          }
        ]
      }
    },
    chapter: "0"
  },
  {
    $schema: "../schema/chapter.zarban.schema.json",
    name: "Chapter 2: Footprints",
    records: {
      tavern_enter: {
        text: [
          "You enter the tavern.",
          "Looking around you see several people of interest milling about the small village bar."
        ],
        effects: [],
        options: [
          {
            prompt: "Take a seat at the bar",
            conditions: [],
            results: [
              "tavern"
            ]
          },
          {
            prompt: "Leave the tavern",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "village_ending_stamina"
              },
              "village"
            ]
          }
        ]
      },
      tavern_ending_alcoholic: {
        text: [
          "You take your familiar seat back at the bar, and order another round.",
          "Having clearly decided to give up vampire hunting for a promising new career in alcoholism, you decide to let Zarban live.",
          "Zarban would later go on to burn down 27 orphanages in your name.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      tavern: {
        text: [
          "You take a seat on a ramshackle stool at the tavern bar."
        ],
        effects: [],
        options: [
          {
            prompt: "Speak to the bartender",
            conditions: [],
            results: [
              "tavern_bartender"
            ]
          },
          {
            prompt: "Approach the suspicious hooded figure lurking in the corner",
            conditions: [
              {
                type: "choices",
                target: "made_dave_go_home",
                value: false
              }
            ],
            results: [
              {
                target: "tavern_lurker",
                conditions: [
                  {
                    type: "choices",
                    target: "made_dave_sad",
                    value: false
                  }
                ]
              },
              {
                target: "tavern_lurker_sad",
                conditions: [
                  {
                    type: "choices",
                    target: "made_dave_sad",
                    value: true
                  }
                ]
              }
            ]
          },
          {
            prompt: "Approach the old farmer drinking alone at the bar",
            conditions: [],
            results: [
              "tavern_farmer"
            ]
          },
          {
            prompt: "Leave the tavern",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "village_ending_stamina"
              },
              "village"
            ]
          }
        ]
      },
      tavern_bartender: {
        text: [
          "You approach the tired looking bartender, intending to vigorously question the overworked customer-service employee.",
          `"Back for another already? What can I get'cha, stranger?"`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask if any strangers have come through town recently",
            conditions: [],
            results: [
              "tavern_bartender_strangers"
            ]
          },
          {
            prompt: "Ask if anyone in town sells weapons and armor",
            conditions: [],
            results: [
              "tavern_bartender_blacksmith"
            ]
          },
          {
            prompt: "I could use a refreshing drink",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "alcoholism",
                    value: 1,
                    operation: "eq"
                  }
                ],
                target: "tavern_ending_alcoholic"
              },
              "tavern_bartender_shots"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_bartender_strangers: {
        text: [
          "You ask the bartender about other strangers that have passed through town.",
          "",
          `"Besides yourself, m'lord? Just one, earlier this very morn'! He'll be long gone by now though, headed out in something of a hurry."`,
          `"Ye could always ask Gaylen, the priest 'round' these parts - his shack'll be a good halfday's walk from here, north of the village."`
        ],
        effects: [
          {
            type: "choices",
            target: "learnt_about_priest",
            value: true
          }
        ],
        options: [
          {
            prompt: "Ask the bartender more questions",
            conditions: [],
            results: [
              "tavern_bartender"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_bartender_blacksmith: {
        text: [
          "You ask the bartender where you might acquire new weapons and armor.",
          "",
          `"Well, you're not likely to find a big fancy smithy in our little village, m'lord"`,
          `"That said, Dave over yonder has been known to fix a rake in his time, if you catch my meanin'"`,
          "The bartender laughs at his attempted joke, as he points to a hooded figure drinking alone in the corner of the bar."
        ],
        effects: [],
        options: [
          {
            prompt: "Ask the bartender more questions",
            conditions: [],
            results: [
              "tavern_bartender"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_bartender_shots: {
        text: [
          "You ask the bartender for a glass of his strongest brew, and down it in once mighty gulp.",
          "Your stamina has been restored"
        ],
        effects: [
          {
            type: "status",
            target: "stamina",
            operation: "add",
            value: 99
          },
          {
            type: "status",
            target: "alcoholism",
            operation: "add",
            value: 1
          }
        ],
        options: [
          {
            prompt: "Ask the bartender more questions",
            conditions: [],
            results: [
              "tavern_bartender"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_lurker: {
        text: [
          "You approach the hooded stranger, clearly an imposing figure. The hulking goliath of a man looks up at you with a scowl on his face.",
          `"What do ye' want, stranger. I ain't in no mood fer conversin' with the likes of you today."`
        ],
        effects: [],
        options: [
          {
            prompt: "I KNOW YOU'RE THE VAMPIRE, YOU MONSTER! PREPARE TO DIE!",
            conditions: [],
            results: [
              "tavern_lurker_sad"
            ]
          },
          {
            prompt: "Ask where you might be able to acquire some new weapons and armor",
            conditions: [],
            results: [
              "tavern_lurker_smithy"
            ]
          },
          {
            prompt: "Ask if he's seen anything unusual",
            conditions: [],
            results: [
              "tavern_lurker_unusual"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_lurker_smithy: {
        text: [
          `"Well I haven't the skills to make such a thing, if that's what yer asking, but I was a soldier in me youth."`,
          '"Meet me at my house just outside the village. Could use a hand with something, then I think I can help ye."'
        ],
        effects: [
          {
            type: "choices",
            target: "made_dave_go_home",
            value: true
          }
        ],
        options: [
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_lurker_unusual: {
        text: [
          `"Wouldn't know nothin' about that, stranger. Leave me be."`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask him more questions",
            conditions: [],
            results: [
              "tavern_lurker"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_lurker_sad: {
        text: [
          "The large muscular man before you begins to cry uncontrollably, tears streaming down his bearded visage.",
          `"Just leave me alone! You're really mean... G... Go away..."`,
          "",
          "The mighty Zarban would never debase himself so. This is clearly not he."
        ],
        effects: [
          {
            type: "choices",
            target: "made_dave_sad",
            value: true
          }
        ],
        options: [
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_farmer: {
        text: [
          "An old man sits at the bar next to you, his skin wrinkled from years of hard labour under the sun."
        ],
        effects: [],
        options: [
          {
            prompt: "Ask him how a farmer has time to drink in broad daylight",
            conditions: [],
            results: [
              "tavern_farmer_sun"
            ]
          },
          {
            prompt: "Make small talk",
            conditions: [],
            results: [
              "tavern_farmer_daughter"
            ]
          },
          {
            prompt: "Ask if anyone in town sells weapons and armor",
            conditions: [],
            results: [
              "tavern_farmer_blacksmith"
            ]
          },
          {
            prompt: "Ask if he's seen anything unusual",
            conditions: [],
            results: [
              "tavern_farmer_unusual"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_farmer_sun: {
        text: [
          '"Never much cared for the sun is all. Do most of me work by night these days. What business is it of yours, anyway?"'
        ],
        effects: [],
        options: [
          {
            prompt: "Continue talking to the farmer",
            conditions: [],
            results: [
              "tavern_farmer"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_farmer_unusual: {
        text: [
          `"Unusual? Two strangers in one night is unusual. It's also annoying. Can I get back to me drink now?"`
        ],
        effects: [],
        options: [
          {
            prompt: "Continue talking to the farmer",
            conditions: [],
            results: [
              "tavern_farmer"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [
              {
                type: "status",
                target: "alcoholism",
                value: 2,
                operation: "lt"
              }
            ],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_farmer_daughter: {
        text: [
          '"Not much of interest happens round these parts, stranger."',
          `"Me daughter's been aweful moody lately, but tis the norm for the young'uns these days."`
        ],
        effects: [],
        options: [
          {
            prompt: "Continue talking to the farmer",
            conditions: [],
            results: [
              "tavern_farmer"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      tavern_farmer_blacksmith: {
        text: [
          `"You could try Dave, round back. He's the hooded feller with crippling emotional issues."`,
          `"He's been known to fix a rake in his time, if you catch my meanin'"`,
          "The farmer laughs heartily, clearly pleased with his joke."
        ],
        effects: [],
        options: [
          {
            prompt: "Continue talking to the farmer",
            conditions: [],
            results: [
              "tavern_farmer"
            ]
          },
          {
            prompt: "Go back to your stool",
            conditions: [],
            results: [
              "tavern"
            ]
          }
        ]
      },
      village_ending_stamina: {
        text: [
          "Exhausted and thirsty, you collapse to the ground. The villagers find you, and bring you to the inn to recover.",
          "Unfortunately, by then Zarban is long gone, and the trail cold. He will later go on to become the CEO of Nestle.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      village: {
        text: [
          "You stand on the main road crossing the village. A nearby sign identifies the town as Rothsten."
        ],
        effects: [
          {
            type: "status",
            target: "stamina",
            operation: "add",
            value: -1
          }
        ],
        options: [
          {
            prompt: "Search the farm",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "village_ending_stamina"
              },
              "village_farm"
            ]
          },
          {
            prompt: "Search Dave's house",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "village_ending_stamina"
              },
              {
                conditions: [
                  {
                    type: "choices",
                    target: "made_dave_go_home",
                    value: false
                  }
                ],
                target: "village_dave_not_home"
              },
              {
                conditions: [
                  {
                    type: "choices",
                    target: "made_dave_sad",
                    value: true
                  }
                ],
                target: "village_dave_not_home"
              },
              "village_dave"
            ]
          },
          {
            prompt: "Search the school",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "village_ending_stamina"
              },
              "village_school_outside"
            ]
          },
          {
            prompt: "Search the mill",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "village_ending_stamina"
              },
              "village_mill"
            ]
          },
          {
            prompt: "Search the tavern",
            conditions: [],
            results: [
              "tavern_enter"
            ]
          },
          {
            prompt: "Leave the village and head to the priest's cottage",
            conditions: [
              {
                type: "choices",
                target: "learnt_about_priest",
                value: true
              }
            ],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "village_ending_stamina"
              },
              "church_bear"
            ]
          }
        ]
      },
      village_farm: {
        text: [
          "You arrive at a well maintained farmhouse. You walk into the house, and see a young woman crushing herbs with a mortar & pestle.",
          "Based on her attire - and the visibility of her assets - you wonder if her clothes were the victim of a shrinking spell.",
          "She does not seem surprised to see you, not so much as looking up from her work.",
          '"A vampire hunter in our little corner of the world? What could possibly bring you here?" She asks you.'
        ],
        effects: [],
        options: [
          {
            prompt: "Speak to the young woman",
            conditions: [],
            results: [
              "village_farm_amelie"
            ]
          },
          {
            prompt: "Look around the humble farmhouse",
            conditions: [],
            results: [
              "village_farm_search"
            ]
          },
          {
            prompt: "Leave the farm",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_farm_amelie: {
        text: [
          '"Need a potion? Or a salve?" The young woman asks'
        ],
        effects: [],
        options: [
          {
            prompt: "Ask her for a potion of vampire detection",
            conditions: [],
            results: [
              "village_farm_amelie_potion"
            ]
          },
          {
            prompt: "Ask her about her family",
            conditions: [],
            results: [
              "village_farm_amelie_family"
            ]
          },
          {
            prompt: "Ask her about herself",
            conditions: [],
            results: [
              "village_farm_amelie_self"
            ]
          },
          {
            prompt: "Ask if she's seen anything unusual",
            conditions: [],
            results: [
              "village_farm_amelie_unusual"
            ]
          },
          {
            prompt: "Stop speaking to the woman",
            conditions: [],
            results: [
              "village_farm"
            ]
          }
        ]
      },
      village_farm_amelie_potion: {
        text: [
          `The woman looks at you confused; "Is that a real potion? I'm afraid I wouldn't know how to make this." The young woman replies`,
          '"I have a potion to cure rhumatism, if that interests you, or the best salve for warts this side of the continent!"'
        ],
        effects: [],
        options: [
          {
            prompt: "Ask her something else",
            conditions: [],
            results: [
              "village_farm_amelie"
            ]
          }
        ]
      },
      village_farm_amelie_unusual: {
        text: [
          '"Unusual?" The young woman laughs',
          `"Honey, having a stranger on our farm is the most unusual thing that's happened in recent memory."`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask her something else",
            conditions: [],
            results: [
              "village_farm_amelie"
            ]
          }
        ]
      },
      village_farm_amelie_family: {
        text: [
          `"Our family? It's an interesting enough tale, I suppose." The young woman begins`,
          '"My father is Arnoulf, of House Brolette - yes those Brolettes."',
          `"In the days of the Aremeic Order, our family laid claim to the thrones of half the Order's colonial territories on the East continent!"`,
          `"My father doesn't like to talk about it, and there's not much left of the family, but it's a proud history to bear."`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask her something else",
            conditions: [],
            results: [
              "village_farm_amelie"
            ]
          }
        ]
      },
      village_farm_amelie_self: {
        text: [
          `"Me? I am Amelie of House Brolette; I'm the village herbalist, not that my skills net me much respect in this hole of a village."`,
          '"I provide these simpletons with medicines and salves, but nobody can see past my choice of clothes. Nobody would bat an eye in the cities."',
          `"Ever try foraging for mushrooms in 3 layers of frill and fuss? You'd never catch me in some silly long dress."`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask her something else",
            conditions: [],
            results: [
              "village_farm_amelie"
            ]
          }
        ]
      },
      village_farm_search: {
        text: [
          "You take a good look around the room.",
          "In the corner are a pair of small beds, one of which clearly hasn't seem much use of late.",
          "In the kitchen, various herbs and mushrooms hang from the ceiling.",
          "Supplies for alchemical brewing are strewn about."
        ],
        effects: [],
        options: [
          {
            prompt: "Speak to the young woman",
            conditions: [],
            results: [
              "village_farm_amelie"
            ]
          },
          {
            prompt: "Leave the farm",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_dave_not_home: {
        text: [
          "You arrive at Dave's home at the edge of the village, a once beautiful home, but clearly neglected of late.",
          "Vines and weeds grow rampant across the property, and the windows sit greased and dusty.",
          "",
          "No candles burn within, and the door is locked tight."
        ],
        effects: [],
        options: [
          {
            prompt: "Leave Dave's house",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_dave: {
        text: [
          "You arrive at Dave's home at the edge of the village, a once beautiful home, but clearly neglected of late.",
          "Vines and weeds grow rampant across the property, and the windows sit greased and dusty.",
          "",
          "The door sits open, so you walk inside. Dave sits alone on a dusty old wooden chair.",
          "All around him sits dusty disused brushes, paints and canvases"
        ],
        effects: [],
        options: [
          {
            prompt: "Talk to Dave",
            conditions: [],
            results: [
              "village_dave_dave"
            ]
          },
          {
            prompt: "Leave Dave's house",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_dave_dave: {
        text: [
          `"So, ye needed a weapon, ey? I do have me gear from the war that I'd be happy enough to never see again."`,
          `"Not usually in the habit of armin' strangers though, so ye'd best have a good cause."`,
          `"My wife, god rest 'er soul didn't approve of wanton violence."`
        ],
        effects: [],
        options: [
          {
            prompt: "Tell him you are hunting a vampire hiding amongst the villagers",
            conditions: [],
            results: [
              "village_dave_dave_honest"
            ]
          },
          {
            prompt: "Tell him you are avenging the death of a loved one",
            conditions: [],
            results: [
              "village_dave_dave_lie"
            ]
          },
          {
            prompt: "Tell him to give you the gear or you'll take it by force",
            conditions: [],
            results: [
              "village_dave_dave_threaten"
            ]
          },
          {
            prompt: "Leave Dave's house",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_dave_dave_honest: {
        text: [
          "Dave sits in silence for an awkwardly long time.",
          `"There's something in the air today; as if the sky itself were thick with dread."`,
          `"I believe you, stranger. My sword and armor are yours. They ain't much but I hope they help you."`,
          '"All I ask is that you lay these here flowers at my family grave. The walk to the temple is too much for this old soldier."',
          `"Ye'll see it marked by the clan name Audemars."`,
          "",
          "The rusted old blade would never be able to strike a killing blow against such a powerful vampire;",
          "But it may grant some protection, at the very least."
        ],
        effects: [
          {
            type: "choices",
            target: "made_dave_sad",
            value: true
          },
          {
            type: "inventory",
            target: "rusty_sword",
            value: true
          },
          {
            type: "inventory",
            target: "old_armor",
            value: true
          },
          {
            type: "inventory",
            target: "dave_flowers",
            value: true
          }
        ],
        options: [
          {
            prompt: "Leave Dave's house",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_dave_dave_lie: {
        text: [
          "Dave slowly shakes his head.",
          '"Sorry stranger, I cannae in good conscience lend creed to vengence. I hope you find a way to see another path."',
          `"In the meantime, I'm afraid ye'll need to leave."`
        ],
        effects: [
          {
            type: "choices",
            target: "made_dave_sad",
            value: true
          }
        ],
        options: [
          {
            prompt: "Leave Dave's house",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_dave_dave_threaten: {
        text: [
          "Without any warning, the giant of a man hits you on the side of the head with one mighty punch.",
          "You awaken outside the house, with the door now securely locked."
        ],
        effects: [
          {
            type: "choices",
            target: "made_dave_sad",
            value: true
          }
        ],
        options: [
          {
            prompt: "Leave Dave's house",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_mill: {
        text: [
          "You arive at the small village mill, and step inside",
          "The stone brick building is small, but impressive - clearly much older than the wooden structures you've seen thus far.",
          "Inside are a man, expertly kneading a large ball of dough, and a woman on a wooden rocking chair repairing a tunic.",
          "",
          `"Ah, we heard there were strangers about!" says the man; "Something ye' knead? Some bread for the road perhaps?"`
        ],
        effects: [],
        options: [
          {
            prompt: "Take the man up on his offer of bread",
            conditions: [
              {
                type: "choices",
                target: "accepted_bread",
                value: false
              }
            ],
            results: [
              "village_mill_bread"
            ]
          },
          {
            prompt: "Ask if they have garlic bread instead",
            conditions: [
              {
                type: "choices",
                target: "accepted_bread",
                value: false
              }
            ],
            results: [
              "village_mill_garlic"
            ]
          },
          {
            prompt: "Ask about their family",
            conditions: [],
            results: [
              "village_mill_family"
            ]
          },
          {
            prompt: "Ask if they've seen anything unusual",
            conditions: [],
            results: [
              "village_mill_unusual"
            ]
          },
          {
            prompt: "Leave the mill",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_mill_family: {
        text: [
          `"We're the Fouchers sir - Nils and Silvia. Our family's been in these parts since the colonial days of the Order."`,
          `"We've 3 little 'uns - Dylan is 8, and the twins are fresh out the oven."`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask something else",
            conditions: [],
            results: [
              "village_mill"
            ]
          }
        ]
      },
      village_mill_unusual: {
        text: [
          '"Not a thing of interest has happened in these here parts in many a year, sir."',
          `"If it's adventure ye' seek, you've picked the wrong village I'm afraid."`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask something else",
            conditions: [],
            results: [
              "village_mill"
            ]
          }
        ]
      },
      village_mill_garlic: {
        text: [
          "You narrow your eyes in suspicion as the question leaves your lips",
          `The man shakes his head; "Nay, stranger. I've a terrible allergy to garlic, so none of that 'ere."`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask something else",
            conditions: [],
            results: [
              "village_mill"
            ]
          }
        ]
      },
      village_mill_bread: {
        text: [
          "The baker hands you a fresh loaf of bread, hot out of the oven,",
          "The family then looks on in horror as you devour the entire loaf in one go like some kind of feral beast"
        ],
        effects: [
          {
            type: "choices",
            target: "accepted_bread",
            value: true
          },
          {
            type: "status",
            target: "stamina",
            operation: "add",
            value: 99
          }
        ],
        options: [
          {
            prompt: "Ask something else",
            conditions: [],
            results: [
              "village_mill"
            ]
          }
        ]
      },
      village_school_outside: {
        text: [
          "You arrive outside the small schoolhouse. A humble shack sits next to it, likely a home for the teacher and her family.",
          "A gaggle of annoying looking children mill about uselessly outside the school."
        ],
        effects: [],
        options: [
          {
            prompt: "Search the shack",
            conditions: [],
            results: [
              "village_school_shack"
            ]
          },
          {
            prompt: "Search the schoolhouse",
            conditions: [],
            results: [
              "village_school_inside"
            ]
          },
          {
            prompt: "Harass the annoying children",
            conditions: [],
            results: [
              "village_school_children"
            ]
          },
          {
            prompt: "Leave the school",
            conditions: [],
            results: [
              "village"
            ]
          }
        ]
      },
      village_school_inside: {
        text: [
          "You walk into the schoolhouse.",
          "A woman - the teacher, presumably - is sat at the far side of the room.",
          "On her head sits the biggest hat you have ever seen. This sunhat has a wingspan that could make a dragon jealous."
        ],
        effects: [],
        options: [
          {
            prompt: "Ask her about herself",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "choices",
                    target: "insulted_teacher",
                    value: true
                  }
                ],
                target: "village_school_items"
              },
              "village_school_teacher"
            ]
          },
          {
            prompt: "Ask if she's seen anything unusual",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "choices",
                    target: "insulted_teacher",
                    value: true
                  }
                ],
                target: "village_school_items"
              },
              "village_school_unusual"
            ]
          },
          {
            prompt: "Ask about the strange items in her home",
            conditions: [
              {
                type: "choices",
                target: "snooped_on_teacher",
                value: true
              }
            ],
            results: [
              "village_school_items"
            ]
          },
          {
            prompt: "Ask about her gigantic hat",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "choices",
                    target: "insulted_teacher",
                    value: true
                  }
                ],
                target: "village_school_items"
              },
              "village_school_hat"
            ]
          },
          {
            prompt: "Look somewhere else",
            conditions: [],
            results: [
              "village_school_outside"
            ]
          }
        ]
      },
      village_school_items: {
        text: [
          `"What kind of creepy imbecile looks in a young woman's windows at her private things?"`,
          '"Leave my classroom this instant!"'
        ],
        effects: [
          {
            type: "choices",
            target: "insulted_teacher",
            value: true
          }
        ],
        options: [
          {
            prompt: "Ask something else",
            conditions: [],
            results: [
              "village_school_inside"
            ]
          }
        ]
      },
      village_school_unusual: {
        text: [
          '"Besides the usual nonsense from that horrible Brolette girl? Not in a long time."'
        ],
        effects: [],
        options: [
          {
            prompt: "Ask something else",
            conditions: [],
            results: [
              "village_school_inside"
            ]
          }
        ]
      },
      village_school_teacher: {
        text: [
          `"My family? nothing special, I'm afraid. Lost my eldest and my husband two winters back to red fever."`,
          '"Milton is outside with his friends - mercifully he was too young to remember it."'
        ],
        effects: [],
        options: [
          {
            prompt: "Ask something else",
            conditions: [],
            results: [
              "village_school_inside"
            ]
          }
        ]
      },
      village_school_hat: {
        text: [
          "The young woman laughs as she grips her cap",
          `"I've a skin condition, always have - the sun; my skin burns so easily"`,
          '"I use the hat to protect myself, along with a salve I make at home!"',
          `"I could buy it from that Brolette girl, but I'd rather not have to interact with that one."`
        ],
        effects: [],
        options: [
          {
            prompt: "Ask something else",
            conditions: [],
            results: [
              "village_school_inside"
            ]
          }
        ]
      },
      village_school_shack: {
        text: [
          "The shack is locked but you look in through the window.",
          "You can see strange objects strewn about - a dagger, various herbs, an ornate pendant covered in runes, and a pouch of white powder"
        ],
        effects: [
          {
            type: "choices",
            target: "snooped_on_teacher",
            value: true
          }
        ],
        options: [
          {
            prompt: "Look somewhere else",
            conditions: [],
            results: [
              "village_school_outside"
            ]
          }
        ]
      },
      village_school_children: {
        text: [
          "You walk up to the children and kick sand at them like a complete arsehole.",
          "One child begins to try, while another kicks you in the groin and runs away giggling."
        ],
        effects: [
          {
            type: "status",
            target: "stamina",
            operation: "add",
            value: -1
          }
        ],
        options: [
          {
            prompt: "Look somewhere else",
            conditions: [],
            results: [
              "village_school_outside"
            ]
          }
        ]
      }
    },
    chapter: "1"
  },
  {
    $schema: "../schema/chapter.zarban.schema.json",
    name: "Chapter 3: The Hunter, Hunted",
    records: {
      church_bear: {
        text: [
          "You head down the wooded path towards the temple.",
          "As you enter the forest, the air around you grows cold and the sun itself begins to dim.",
          "",
          "You hear a mighty roar from behind you."
        ],
        effects: [],
        options: [
          {
            prompt: "Turn around and face your foe",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "rusty_sword",
                    value: true
                  }
                ],
                target: "church_bear_armed"
              },
              "church_bear_unarmed"
            ]
          },
          {
            prompt: "Run away from the creature",
            conditions: [],
            results: [
              "church_bear_coward"
            ]
          }
        ]
      },
      church_bear_coward: {
        text: [
          "You bravely run away, as fast as you can into the woods, towards the temple, and Gaylen the town priest.",
          "You arrive at the temple late in the afternoon, and see the priest standing outside the ornate stone building.",
          "Nearby you can see a graveyard, with a massive ornate tomb at its center.",
          "The priest approches you:",
          '"Greetings, stranger. How may I be of service?"'
        ],
        effects: [],
        options: [
          {
            prompt: "Speak to the priest",
            conditions: [],
            results: [
              "church_gaylen"
            ]
          }
        ]
      },
      church_bear_unarmed: {
        text: [
          "You turn to face your foe and come face to crotch with a 12ft tall monster of a bear.",
          "You instinctively reach for your blade, but of course, you don't have one.",
          "You start to run away, but the mighty beast - no doubt the shapeshifter himself - rips you open with a mighty swipe of his paw",
          "",
          "It will be daybreak before you are found by the villagers, and after numerous surgeries and expensive physiotherapy you regain the ability to walk",
          "But by then, when no one was looking, Zarban took forty cakes.",
          "That's as many as four tens.",
          "And that's terrible.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      church_bear_armed: {
        text: [
          "You turn to face your foe and come face to crotch with a 12ft tall monster of a bear.",
          "You instinctively reach for your blade, and draw the corroded shortsword you were given.",
          "The mighty beast looms over you, menace in his eyes. The beast drips with magic and malice;",
          "You know in your heart that can only be Zarban the undead shapeshifter himself."
        ],
        effects: [],
        options: [
          {
            prompt: "Run away very very quickly",
            conditions: [],
            results: [
              "church_bear_armed_coward"
            ]
          },
          {
            prompt: "Go for the beast's head",
            conditions: [],
            results: [
              "church_bear_armed_stupid"
            ]
          },
          {
            prompt: "Slash at the beasts legs",
            conditions: [],
            results: [
              "church_bear_armed_legs"
            ]
          }
        ]
      },
      church_bear_armed_coward: {
        text: [
          "You start to run away, but the mighty beast rips your back open with a mighty swipe of his paw",
          "",
          "It will be daybreak before you are found by the villagers, and after numerous surgeries and expensive physiotherapy you regain the ability to walk",
          "But by then, Zarban is long gone. He would later go on to start a mediocre reaction channel on TikTok.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      church_bear_armed_stupid: {
        text: [
          "You stab directly upwards into the beasts head, aiming to finish this now and here.",
          "The common steel of the blade cannot deal a finishing blow against such a powerful magical foe, however and the metal passes right through the beast.",
          "While you leave your flank fully exposed, Zarban disembowels you with ease.",
          "Zarban would later go on to become a successful corporate attorney.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      church_bear_armed_legs: {
        text: [
          "You slash at the beast's legs, causing him to roar in pain. You can't kill him with this sword, but even if he takes on another form,",
          "It will take days for him to heal from such a blow."
        ],
        effects: [
          {
            type: "choices",
            target: "injured_zarban",
            value: true
          }
        ],
        options: [
          {
            prompt: "Escape while the beast reels from the blow",
            conditions: [],
            results: [
              "church_bear_armed_escape"
            ]
          },
          {
            prompt: "Go for the beast's head",
            conditions: [],
            results: [
              "church_bear_armed_stupid"
            ]
          }
        ]
      },
      church_bear_armed_escape: {
        text: [
          "With the beast's legs injured, you take your chance to escape into the woods, towards the temple, and Gaylen the town priest.",
          "You arrive at the temple late in the afternoon, and see the priest standing outside the ornate stone building.",
          "Nearby you can see a graveyard, with a massive ornate tomb at its center.",
          "The priest approches you:",
          '"Greetings, stranger. How may I be of service?"'
        ],
        effects: [],
        options: [
          {
            prompt: "Speak to the priest",
            conditions: [],
            results: [
              "church_gaylen"
            ]
          },
          {
            prompt: "There is nothing for me here, let's return to town",
            conditions: [
              {
                type: "inventory",
                target: "rusty_sword",
                value: true
              }
            ],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      church_gaylen: {
        text: [
          '"Yes, strenger, how can I help you?"'
        ],
        effects: [],
        options: [
          {
            prompt: "Ask for help against the evil vampire",
            conditions: [
              {
                type: "inventory",
                target: "ancient_armor",
                value: false
              }
            ],
            results: [
              {
                conditions: [
                  {
                    type: "choices",
                    target: "impressed_gaylen",
                    value: true
                  }
                ],
                target: "church_gaylen_success"
              },
              "church_gaylen_fail"
            ]
          },
          {
            prompt: "Ask about the temple",
            conditions: [],
            results: [
              "church_gaylen_temple"
            ]
          },
          {
            prompt: "Ask the priest about Dave's family grave",
            conditions: [
              {
                type: "inventory",
                target: "dave_flowers",
                value: true
              }
            ],
            results: [
              "church_gaylen_impressed"
            ]
          },
          {
            prompt: "Go to the graveyard",
            conditions: [],
            results: [
              "church_graveyard"
            ]
          }
        ]
      },
      church_gaylen_success: {
        text: [
          "The priest stays silent for a time, appearing to pray to himself. Finally he speaks;",
          '"As you are a friend to the village, and an honourable soul, I grant you the protection of the great Edwin Rothsten."',
          `"May the ancient vampire hunter's armor protect you. And take this key, you'll find Moonsbane in the hunter's tomb."`
        ],
        effects: [
          {
            type: "inventory",
            target: "ancient_armor",
            value: true
          },
          {
            type: "inventory",
            target: "hunter_tomb_key",
            value: true
          },
          {
            type: "inventory",
            target: "old_armor",
            value: false
          },
          {
            type: "choices",
            target: "learnt_about_hunter",
            value: true
          }
        ],
        options: [
          {
            prompt: "Ask the priest something else",
            conditions: [],
            results: [
              "church_gaylen"
            ]
          },
          {
            prompt: "Go to the graveyard",
            conditions: [],
            results: [
              "church_graveyard"
            ]
          }
        ]
      },
      church_gaylen_fail: {
        text: [
          '"I will say a prayer of blessing for you stranger."',
          '"Sadly I have only your word, so I cannot help you more than that. Good luck to you."'
        ],
        effects: [],
        options: [
          {
            prompt: "Ask the priest something else",
            conditions: [],
            results: [
              "church_gaylen"
            ]
          },
          {
            prompt: "Go to the graveyard",
            conditions: [],
            results: [
              "church_graveyard"
            ]
          }
        ]
      },
      church_gaylen_temple: {
        text: [
          '"This temple was founded over 700 years ago during the time of the Aremeic Order."',
          '"It was personally commisioned by the famed vampire hunter, and knight of the order Edwin Rothsten."',
          '"The legend himself is entombed here, where himself and his enchanted blade Moonsbane can watch over us."',
          "",
          "The priest points to the ornate tomb in the center of the graveyard - you think to yourself that it might be prudent to 'borrow' Moonsbane."
        ],
        effects: [
          {
            type: "choices",
            target: "learnt_about_hunter",
            value: true
          }
        ],
        options: [
          {
            prompt: "Ask the priest something else",
            conditions: [],
            results: [
              "church_gaylen"
            ]
          },
          {
            prompt: "Go to the graveyard",
            conditions: [],
            results: [
              "church_graveyard"
            ]
          }
        ]
      },
      church_gaylen_impressed: {
        text: [
          "You show the flowers to the priest, who smiles warmly at you.",
          '"Ah! A friend of lord Audemars is a friend of mine. I will bless these and place them at his family grave for you."'
        ],
        effects: [
          {
            type: "inventory",
            target: "dave_flowers",
            value: false
          },
          {
            type: "choices",
            target: "impressed_gaylen",
            value: true
          }
        ],
        options: [
          {
            prompt: "Ask the priest something else",
            conditions: [],
            results: [
              "church_gaylen"
            ]
          },
          {
            prompt: "Go to the graveyard",
            conditions: [],
            results: [
              "church_graveyard"
            ]
          }
        ]
      },
      church_graveyard: {
        text: [
          "You arrive at the humble graveyard. In its center lies an ornate tomb marked 'Edwin Rothsten, Hunter of the Aremeic Order'",
          "The family grave of the Audemars clan sits nearby."
        ],
        effects: [],
        options: [
          {
            prompt: "Place the flower's on Dave Audemars' family grave",
            conditions: [
              {
                type: "inventory",
                target: "dave_flowers",
                value: true
              }
            ],
            results: [
              "church_graveyard_flowers"
            ]
          },
          {
            prompt: "Retrieve the sword from the hunter's tomb",
            conditions: [
              {
                type: "choices",
                target: "learnt_about_hunter",
                value: true
              }
            ],
            results: [
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "hunter_tomb_key",
                    value: true
                  }
                ],
                target: "church_graveyard_key"
              },
              "church_graveyard_breakin"
            ]
          },
          {
            prompt: "Return to the village, and confront Zarban",
            conditions: [
              {
                type: "inventory",
                target: "rusty_sword",
                value: true
              }
            ],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      church_graveyard_flowers: {
        text: [
          "You place the flowers at the foot of the family grave stone, and say a short prayer.",
          "You feel content, your promise fulfilled."
        ],
        effects: [
          {
            type: "inventory",
            target: "dave_flowers",
            value: false
          }
        ],
        options: [
          {
            prompt: "Retrieve the sword from the hunter's tomb",
            conditions: [
              {
                type: "choices",
                target: "learnt_about_hunter",
                value: true
              }
            ],
            results: [
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "hunter_tomb_key",
                    value: true
                  }
                ],
                target: "church_graveyard_key"
              },
              "church_graveyard_breakin"
            ]
          },
          {
            prompt: "Return to the village, and confront Zarban",
            conditions: [
              {
                type: "inventory",
                target: "rusty_sword",
                value: true
              }
            ],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      church_graveyard_key: {
        text: [
          "You walk up to the tomb, and use a the key to open the ornate lock.",
          "You pick up the ancient runic sword and feel its holy power flow through you - Zarban's end is at hand!"
        ],
        effects: [
          {
            type: "inventory",
            target: "magic_sword",
            value: true
          },
          {
            type: "inventory",
            target: "rusty_sword",
            value: false
          },
          {
            type: "inventory",
            target: "hunter_tomb_key",
            value: false
          }
        ],
        options: [
          {
            prompt: "Return to the village, and confront Zarban",
            conditions: [],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      church_graveyard_breakin: {
        text: [
          "You walk up to the tomb, and use a nearby rock to smash open the ornate lock.",
          "You pick up the ancient runic sword and equip it - Zarban's end is at hand!",
          "You'll probably return it when you are finished. Maybe."
        ],
        effects: [
          {
            type: "inventory",
            target: "magic_sword",
            value: true
          },
          {
            type: "inventory",
            target: "rusty_sword",
            value: false
          }
        ],
        options: [
          {
            prompt: "Return to the village, and confront Zarban",
            conditions: [],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      }
    },
    chapter: "2"
  },
  {
    $schema: "../schema/chapter.zarban.schema.json",
    name: "Chapter 4: J'Accuse!",
    records: {
      jaccuse_arrival: {
        text: [
          "You arrive back in the village at midnight. The stench of evil still hangs in the air - it is time to confront Zarban.",
          "But which villager has been replaced by the vile sorcerer?"
        ],
        effects: [
          {
            type: "status",
            target: "stamina",
            operation: "add",
            value: -1
          }
        ],
        options: [
          {
            prompt: "Go to the tavern",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "jaccuse_end_stamina"
              },
              "jaccuse_tavern"
            ]
          },
          {
            prompt: "Go to the farm",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "jaccuse_end_stamina"
              },
              {
                conditions: [
                  {
                    type: "choices",
                    target: "injured_zarban",
                    value: true
                  }
                ],
                target: "jaccuse_farm_scar"
              },
              "jaccuse_farm"
            ]
          },
          {
            prompt: "Go to Dave's house",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "jaccuse_end_stamina"
              },
              "jaccuse_dave"
            ]
          },
          {
            prompt: "Go to the schoolhouse",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "status",
                    target: "stamina",
                    operation: "eq",
                    value: 0
                  }
                ],
                target: "jaccuse_end_stamina"
              },
              "jaccuse_school"
            ]
          }
        ]
      },
      jaccuse_end_stamina: {
        text: [
          "Exhausted and thirsty, you collapse to the ground. The villagers find you, and bring you to the inn to recover.",
          "Unfortunately, by then Zarban is long gone, and the trail cold. He will later go on to form a knitting circle that only makes ugly sweaters for puppies.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_tavern: {
        text: [
          "You enter the tavern. The bartender stands at his usual spot, and Dave drinks alone in the corner."
        ],
        effects: [],
        options: [
          {
            prompt: "Confront the bartender",
            conditions: [],
            results: [
              "jaccuse_bartender"
            ]
          },
          {
            prompt: "Confront Dave",
            conditions: [],
            results: [
              "jaccuse_dave"
            ]
          },
          {
            prompt: "Go somewhere else",
            conditions: [],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      jaccuse_bartender: {
        text: [
          "You approach the bartender, who smiles warmly on your approach. The smile fades as you draw your blade.",
          "As your sword plunges into the man's chest, and you see the light leave his eyes, you know in your heart you've chosen wrong.",
          "The innocent man falling to the ground before you is the last thing you ever see as a drunken, angry Dave breaks your neck from behind.",
          "Zarban would later escape, and go on to form a boy band that only sings Nickelback covers.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_dave: {
        text: [
          "Dave meets your gaze as you approach him, the old soldier recognizing the look in your eyes.",
          `A single tear falls down his cheek as he mutters to himself "I'm coming home, Amy."`,
          "As your blade pierces his heart and Dave Audemars dies before you, you know in your heart you've chosen wrong.",
          "A bottle breaks over your head from behind, and you fall to the ground. You'll later awaken just in time for your hanging",
          "Zarban would later escape, and go on to become the world's most successful spam email marketer.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_farm_scar: {
        text: [
          "You arrive at the farm, and find Arnoulf and his daughter working the field by moonlight.",
          "The white frills of Amelie's long dress reflect the dim light of the moon"
        ],
        effects: [],
        options: [
          {
            prompt: "Confront the old farmer",
            conditions: [],
            results: [
              "jaccuse_farm_arnoulf"
            ]
          },
          {
            prompt: "Confront the farmer's daughter",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "rusty_sword",
                    value: true
                  }
                ],
                target: "jaccuse_farm_amelie_nosword"
              },
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "ancient_armor",
                    value: true
                  }
                ],
                target: "jaccuse_farm_amelie_good"
              },
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "magic_sword",
                    value: true
                  }
                ],
                target: "jaccuse_farm_amelie_noarmor"
              }
            ]
          },
          {
            prompt: "Go somewhere else",
            conditions: [],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      jaccuse_farm: {
        text: [
          "You arrive at the farm, and find Arnoulf and his daughter working the field by moonlight."
        ],
        effects: [],
        options: [
          {
            prompt: "Confront the old farmer",
            conditions: [],
            results: [
              "jaccuse_farm_arnoulf"
            ]
          },
          {
            prompt: "Confront the farmer's daughter",
            conditions: [],
            results: [
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "rusty_sword",
                    value: true
                  }
                ],
                target: "jaccuse_farm_amelie_nosword"
              },
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "ancient_armor",
                    value: true
                  }
                ],
                target: "jaccuse_farm_amelie_good"
              },
              {
                conditions: [
                  {
                    type: "inventory",
                    target: "magic_sword",
                    value: true
                  }
                ],
                target: "jaccuse_farm_amelie_noarmor"
              }
            ]
          },
          {
            prompt: "Go somewhere else",
            conditions: [],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      jaccuse_farm_arnoulf: {
        text: [
          "You approach the farmer as he toils under the moonlight, and draw your blade.",
          "The farmer scowls as he raises his pitchfork to confront you.",
          "You manage to get one good slash in on the farmer's neck... just as his pitchfork pierces your lungs.",
          "As you both bleed out on the ground, you know you have chosen incorrectly.",
          "Zarban would later go on to invent a new type of toothpaste that causes cavities instead of preventing them.",
          "",
          "GAME OVER! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_farm_amelie_nosword: {
        text: [
          "You approach the young woman and draw your sword.",
          "She dons an evil grin and her features twist and distort, as the evil shapeshifter assumes to form of a massive bear!",
          "You hack and slash away to no avail, as the common steel of your rusted blade cannot kill the vampire.",
          "As the old farmer tries in vain to help you, your neck is ripped open by his mighty jaws, and you bleed out onto the moonlit field.",
          "Zarban, revealed and unleashed, will draw strength from the blood of the village by massacring the entire populate of the tiny hamlet.",
          "Renewed by his bloodbath, Zarban escapes into the world, more powerful than ever before.",
          "",
          "The End! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_farm_amelie_noarmor: {
        text: [
          "You approach the young woman and draw your sword.",
          "She dons an evil grin and her features twist and distort, as the evil shapeshifter assumes to form of a massive bear!",
          "You stab the vile creature through it's dark heart, but in his dying breath, the evil sorcerer utters a final curse.",
          "As you fall to the ground, the air sucked out of your lungs, and slowly choke to death on your own blood,",
          "You see the evil creature turn to dust before you. You may die, but at least the world is safe from this creature.",
          "The village will remember you as a brave hero.",
          "",
          "The End! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_farm_amelie_good: {
        text: [
          "You approach the young woman and draw your sword.",
          "She dons an evil grin and her features twist and distort, as the evil shapeshifter assumes to form of a massive bear!",
          "You stab the vile creature through it's dark heart, but in his dying breath, the evil sorcerer utters a final curse.",
          "Your enchanted armor glows bright with ancient runic magic as the vampire's curse is nullified.",
          "The creature contorts and screeches as it is reduced to dust and ash - the vile one has been vanquished!",
          "",
          "Congratulations! You are victorious!"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_davehouse: {
        text: [
          "You arrive at Dave's home at the edge of the village, a once beautiful home, but clearly neglected of late.",
          "Vines and weeds grow rampant across the property, and the windows sit greased and dusty.",
          "",
          "No candles burn within, and the door is locked tight."
        ],
        effects: [],
        options: [
          {
            prompt: "Go somewhere else",
            conditions: [],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      jaccuse_school: {
        text: [
          "You kick down the door of the teacher's humble shack and burst in, sword drawn.",
          "The teacher and her 6 year old son scream as you run inside."
        ],
        effects: [],
        options: [
          {
            prompt: "Confront the teacher",
            conditions: [],
            results: [
              "jaccuse_school_teacher"
            ]
          },
          {
            prompt: "Confront the son",
            conditions: [],
            results: [
              "jaccuse_school_teacher_child"
            ]
          },
          {
            prompt: "Go somewhere else",
            conditions: [],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      jaccuse_school_teacher: {
        text: [
          "You expertly swing your blade, and take the young woman's head clean off!",
          "She dies instantly, since she was a schoolteacher - not a vampire.",
          "You run off as the young boy cries and vows to avenge his mother.",
          "You spend the rest of your days in hiding, as Zarban goes on to invent a new type of coffee that tastes like burnt popcorn and expired milk.",
          "",
          "The End! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_school_teacher_child: {
        text: [
          "You expertly swing your blade at the young child, but the mother blocks your blade with her body.",
          "The young woman bleeds out in seconds, and the small cut on the boy's arm reveals the pure red blood of a human child.",
          "You run off as the young boy cries and vows to avenge his mother.",
          "You spend the rest of your days in hiding, as Zarban goes on to open a bakery that only sells cakes made with vegetables instead of sugar.",
          "",
          "The End! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_mill: {
        text: [
          "You burst into the stone building and charge to the far side of the room, where the family sits around a fireplace.",
          "They look up in confused horror at the unexpected home invasion.",
          "A bandage is wrapped around the baker's hand, clearly a fresh wound."
        ],
        effects: [],
        options: [
          {
            prompt: "Confront the baker",
            conditions: [],
            results: [
              "jaccuse_mill_baker"
            ]
          },
          {
            prompt: "Confront the seamstress",
            conditions: [],
            results: [
              "jaccuse_mill_seamstress"
            ]
          },
          {
            prompt: "Confront the 8 year old boy",
            conditions: [],
            results: [
              "jaccuse_mill_boy"
            ]
          },
          {
            prompt: "Confront the twins",
            conditions: [],
            results: [
              "jaccuse_mill_babies"
            ]
          },
          {
            prompt: "Go somewhere else",
            conditions: [],
            results: [
              "jaccuse_arrival"
            ]
          }
        ]
      },
      jaccuse_mill_baker: {
        text: [
          "You draw your blade and eviscerate the baker and father.",
          "As his guts spill all over the floor of the mill, the grieving family holds you down until the authorities arrive.",
          "You will be sentenced to life in prison without parole, from where you will be blissfully unaware of all the puppies Zarban would later eat.",
          "",
          "The End! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_mill_seamstress: {
        text: [
          "You draw your blade and stab the young mother of 3 through the eye.",
          "As she falls to the ground and you realize the gravity of your mistake, a cast iron pan to the forehead puts you out of everyone's misery.",
          "Zarban would later go on to traffic in baby spines for fun and profit.",
          "",
          "The End! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_mill_boy: {
        text: [
          "You draw your sword and slash at the young boy's throat.",
          "The boy uses his arms to shield himself, and survives - barely - your blow.",
          "The pure red blood of a human tells you that a mistake has been made as his mother pierces your brain with a fork through the eye.",
          "Zarban would later go on to rethink his life and become a nun. Just kidding he eats children now.",
          "",
          "The End! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      },
      jaccuse_mill_babies: {
        text: [
          "You draw your sword and approach the sleeping newborn babies.",
          "You stop yourself from murdering two babies at the last second and decide to go to the tavern to rethink your life.",
          "The closecall has scarred you, and you no longer wish to hunt vampires.",
          "Zarban would later to on to invest heavily into dogecoin.",
          "",
          "The End! Try again?"
        ],
        effects: [],
        options: [
          {
            prompt: "New game",
            conditions: [],
            results: [
              "intro_cave1"
            ]
          }
        ]
      }
    },
    chapter: "3"
  }
];
const choices = {
  $schema: "../schema/player.choices.zarban.schema.json",
  records: {
    made_dave_go_home: {
      description: "Spoke to Dave in the tavern, making him go back to his home"
    },
    made_dave_sad: {
      description: "Was rude to Dave, who will no longer help you"
    },
    learnt_about_priest: {
      description: "Learnt about the church a day's walk down the road from the village"
    },
    accepted_bread: {
      description: "Accepted bread from the baker"
    },
    snooped_on_teacher: {
      description: "Snooped on the teacher's shack"
    },
    insulted_teacher: {
      description: "Teacher found out about your snooping"
    },
    impressed_gaylen: {
      description: "Impressed the priest with your kindness"
    },
    learnt_about_hunter: {
      description: "Learnt about the famous hunter (and his sword)"
    },
    injured_zarban: {
      description: "Injured Zarban's legs"
    }
  }
};
const inventory = {
  $schema: "../schema/player.inventory.zarban.schema.json",
  records: {
    hunter_sword: {
      description: "Vampire hunter's sword",
      equipped: false,
      effects: []
    },
    hunter_armor: {
      description: "Vampire hunter's armor",
      equipped: false,
      effects: []
    },
    rusty_sword: {
      description: "Rusty shortsword",
      equipped: false,
      effects: []
    },
    old_armor: {
      description: "Soldier's leather cuirass",
      equipped: false,
      effects: [
        {
          type: "status",
          target: "stamina",
          operation: "add_max",
          value: 2
        },
        {
          type: "status",
          target: "stamina",
          operation: "add",
          value: 2
        }
      ]
    },
    magic_sword: {
      description: "Moonsbane",
      equipped: false,
      effects: []
    },
    ancient_armor: {
      description: "Edwin Rothsten's armor",
      equipped: false,
      effects: [
        {
          type: "status",
          target: "stamina",
          operation: "add_max",
          value: 5
        },
        {
          type: "status",
          target: "stamina",
          operation: "add",
          value: 5
        }
      ]
    },
    hunter_tomb_key: {
      description: "Key to the vampire hunter's tomb",
      equipped: false,
      effects: []
    },
    dave_flowers: {
      description: "Flower's for Dave's family grave",
      equipped: false,
      effects: []
    }
  }
};
const status = {
  $schema: "../schema/player.status.zarban.schema.json",
  records: {
    stamina: {
      hidden: false,
      "default": 0,
      maximum: 3
    },
    alcoholism: {
      hidden: true,
      "default": 0,
      maximum: 2
    }
  }
};
const _player_data = {
  $schema,
  entrypoint,
  chapters,
  choices,
  inventory,
  status
};
let player_data = _player_data;
if (player_data.data) {
  player_data = JsonUtilities.base64Decode(player_data.data);
  player_data = JSON.parse(player_data);
}
class Player {
  constructor(json) {
    JsonUtilities.assign(this, player_data);
    if (json !== void 0) {
      JsonUtilities.assign(this, json);
    }
    this.choices = new PlayerChoices(this.choices);
    this.inventory = new PlayerInventory(this.inventory);
    this.status = new PlayerStatus(this.status);
    for (const i in this.chapters) {
      this.chapters[i] = new Chapter(this.chapters[i]);
    }
    if (this.currentStoryKey === void 0) {
      this.setStory(this.entrypoint);
    }
  }
  /**
   * Return the current story
   * @returns Story
   */
  currentStory() {
    return this.getStory(this.currentStoryKey);
  }
  /**
   * Return the current chapter
   * @returns Chapter
   */
  currentChapter() {
    return this.chapters[this.currentChapterKey];
  }
  /**
   * Return true if this is new game
   * @returns bool
   */
  isNewGame() {
    return this.currentStoryKey == this.entrypoint;
  }
  /**
   * Validate an option's conditions
   * @param {StoryOption} option 
   * @returns bool
   */
  validateConditions(conditions) {
    let stats = this.getAdjustedStats();
    for (const condition of conditions) {
      if (!condition.verify(stats)) {
        return false;
      }
    }
    return true;
  }
  /**
   * Based on a selection, find the next available story
   * @param {Number} option_id 
   * @returns String, or false for invalid selections
   */
  nextStory(option_id) {
    let next = this.currentStory().options.filter((o) => this.validateConditions(o.conditions))[option_id - 1];
    if (next === void 0)
      return false;
    for (const result of next.results) {
      if (!this.validateConditions(result.conditions))
        continue;
      this.setStory(result.target);
      return true;
    }
    return false;
  }
  /**
   * Update current game story
   * @param {String} story_key 
   */
  setStory(story_key) {
    for (const i in Object.keys(this.chapters)) {
      let story = this.chapters[i].getStory(story_key);
      if (story !== false) {
        this.currentChapterKey = i;
        this.currentStoryKey = story_key;
        for (const effect of story.effects) {
          effect.apply(this);
        }
        return true;
      }
    }
    return false;
  }
  /**
   * Get inventory-effect adjusted stats
   * @returns a copy of the player
   */
  getAdjustedStats() {
    let playerCopy = new Player(this);
    for (const effect of this.inventory.activeEffects()) {
      effect.apply(playerCopy);
    }
    return this;
  }
  /**
   * Retrieve a story by ID
   * @param {String} story_key 
   * @returns A Story object, or false
   */
  getStory(story_key) {
    for (const chapter of Object.values(this.chapters)) {
      let story = chapter.getStory(story_key);
      if (story !== false) {
        return story;
      }
    }
    throw new Error(`Invalid story ID referenced: ${story_key}!`);
  }
  /**
   * Save current game status to a string
   * @returns String
   */
  save() {
    const gameData = {
      currentChapterKey: this.currentChapterKey,
      currentStoryKey: this.currentStoryKey,
      status: {},
      choices: this.choices.list_chosen(),
      inventory: this.inventory.list_equipped()
    };
    this.status.list().forEach((i) => {
      gameData.status[i] = {
        value: this.status.get(i).value,
        maximum: this.status.get(i).maximum
      };
    });
    return encodeURIComponent(
      JSON.stringify(gameData)
    );
  }
  /**
   * Restore a game save from the save data
   * @param {String} data 
   * @returns Player
   */
  static restore(data) {
    try {
      const gameData = JSON.parse(
        decodeURIComponent(data)
      );
      const player = new Player({
        currentChapterKey: gameData.currentChapterKey,
        currentStoryKey: gameData.currentStoryKey
      });
      gameData.choices.forEach((k) => {
        player.choices.records[k].enabled = true;
      });
      gameData.inventory.forEach((k) => {
        player.inventory.records[k].equipped = true;
      });
      Object.keys(gameData.status).forEach((k) => {
        player.status.records[k].value = gameData.status[k].value;
        player.status.records[k].maximum = gameData.status[k].maximum;
      });
      return player;
    } catch (e) {
      console.log(e);
      return new Player();
    }
  }
}
class Interface {
  static getTitledBox(title, inner_text_lines) {
    let box_width = Math.max(...inner_text_lines.map((l) => l.length).concat([title.length])) + 2;
    let output = [
      `╔${"═".repeat(box_width)}╗`,
      `║ ${title}${" ".repeat(box_width - title.length - 1)}║`,
      `╟${"─".repeat(box_width)}╢`
    ];
    for (const line of inner_text_lines) {
      output.push(`║ ${line}${" ".repeat(box_width - line.length - 1)}║`);
    }
    output.push(`╚${"═".repeat(box_width)}╝`);
    return output;
  }
  static getPlayerDetailString(player, debug = false) {
    let output = [];
    for (const s of debug ? player.status.list() : player.status.list_visible()) {
      let status2 = player.status.get(s);
      let name2 = s.charAt(0).toUpperCase() + s.slice(1);
      output.push(`${name2} : ${status2.value} / ${status2.maximum}`);
    }
    let items = player.inventory.all_equipped();
    if (items.length) {
      output.push("");
      output.push("Equipment:");
      for (const item of player.inventory.all_equipped()) {
        output.push(`- ${item.description}`);
      }
    }
    if (debug) {
      output.push("");
      output.push("Choices:");
      for (const choice of player.choices.list().filter((c) => player.choices.chose(c))) {
        output.push(`- ${choice}`);
      }
    }
    return output;
  }
  static getInterfaceString(player, has_error = false, debug = false) {
    let playerStats = player.getAdjustedStats();
    let title = playerStats.currentChapter().name;
    let inner_text = [
      "",
      ...playerStats.currentStory().text,
      "",
      ...this.getPlayerDetailString(playerStats, debug)
    ];
    let options = playerStats.currentStory().options.filter((o) => playerStats.validateConditions(o.conditions));
    let prompt = [
      `${has_error ? "Invalid selection. " : ""}What do you do?`,
      ...options.map((o, i) => `${i + 1}) ${o}`)
    ];
    return [
      ...this.getTitledBox(title, inner_text),
      "",
      ...prompt
    ].join("\n");
  }
  static getInterfaceStrings(player) {
    const stats = player.getAdjustedStats();
    return {
      title: stats.currentChapter().name,
      description: [
        ...stats.currentStory().text,
        "",
        ...stats.status.list_visible().map((s) => `${s}: ${stats.status.get(s).value}/${stats.status.get(s).maximum}`),
        "",
        "Equipment:",
        ...stats.inventory.all_equipped().map((i) => `- ${i.description}`)
      ],
      options: stats.currentStory().options.filter((o) => stats.validateConditions(o.conditions))
    };
  }
}
class ZarbanRunner {
  constructor(save_data) {
    this.player = save_data ? Player.restore(save_data) : new Player();
  }
  /**
   * Get the current story text
   */
  getInterfaceStrings() {
    return Interface.getInterfaceStrings(this.player);
  }
  /**
   * Return the current game state
   */
  save() {
    return this.player.save();
  }
  /**
   * Reset the game state
   */
  reset() {
    this.player = new Player();
    this.draw();
  }
  /**
   * Render the current game state
   */
  draw() {
    return "";
  }
  /**
   * Advance the game state
   */
  step(option) {
    this.error = option ? !this.player.nextStory(option) : false;
    return this.draw();
  }
}
class ZarbanLavendeuxRunner extends ZarbanRunner {
  constructor(save_data) {
    super(save_data);
  }
  /**
   * Render the current game state
   */
  draw() {
    const strings = this.getInterfaceStrings();
    const box = Interface.getTitledBox(strings.title, strings.description);
    return [
      "",
      ...box,
      this.error ? "\nInvalid option!" : "",
      "What do you do?",
      ...strings.options.map((o, i) => `${i + 1} @zarban) ${o}`),
      "",
      'You can choose an option from above, such as "1 @zarban" and "zarban(2)" or start a new game with "start/restart @zarban" or "zarban("start")"',
      "Type your selection below and use Lavendeux to continue your adventure!\n"
    ].join("\n");
  }
  /**
   * Register zarban as a lavendeux extension
   */
  static registerExtension(name2, author2, version2) {
    if (!lavendeux)
      return;
    let extension = lavendeux.extend({
      name: name2,
      author: author2,
      version: version2
    });
    extension.addStringDecorator("zarban", ZarbanLavendeuxRunner.callback);
    extension.addStringFunction("zarban", ZarbanLavendeuxRunner.callback).requireStringArgument();
    lavendeux.register(extension);
  }
  /**
   * Callback method for running zarban through lavendeux
   */
  static callback(option, state) {
    if (["start", "restart", ""].includes(option.toLowerCase())) {
      delete state.zarban_save;
    }
    const game = new ZarbanLavendeuxRunner(state.zarban_save);
    const result = game.step(state.zarban_save ? option : false);
    state.zarban_save = game.save();
    return result;
  }
}
ZarbanLavendeuxRunner.registerExtension(name, author, version);
class InlineElement {
  constructor(tag) {
    this.e = document.createElement(tag);
  }
  setAttribute(name2, value) {
    this.e.setAttribute(name2, value);
    return this;
  }
  setInnerHTML(value) {
    this.e.innerHTML = value;
    return this;
  }
  setOnClick(handler) {
    this.e.onclick = handler;
    return this;
  }
  setStyle(name2, value) {
    this.e.style[name2] = value;
    return this;
  }
  element() {
    return this.e;
  }
}
const ZARBAN_GAMEBOARD_CLASS = "zarban_gameboard";
const ZARBAN_CONTROLS_CLASS = "zarban_controls";
class ZarbanWebRunner extends ZarbanRunner {
  constructor(container_id) {
    const data = localStorage[`zarban_${container_id}`];
    super(data);
    this.container = document.getElementById(container_id);
    this.injectCSS(container_id);
    this.initContainer();
    globalThis.advanceGame = (option) => this.step(option);
    advanceGame();
  }
  /**
   * Create the container divs for the game
   */
  initContainer() {
    this.container.dataZarbanRunner = this;
    this.container.innerHTML = "";
    this.container.appendChild(
      new InlineElement("div").setAttribute("class", ZARBAN_GAMEBOARD_CLASS).element()
    );
    this.container.appendChild(
      new InlineElement("div").setAttribute("class", ZARBAN_CONTROLS_CLASS).element()
    );
  }
  /**
   * Inject game style into the page
   */
  injectCSS() {
    let css = document.createElement("style");
    css.type = "text/css";
    css.innerHTML = `
            body,html {
                background-color: #333; color: #00AA20;
                font-family: "Lucida Console", "Courier New", monospace; font-size: 1em;
            }
            .zarban_gameboard {
                border: 1px solid #00AA20; border-width: 4px; border-style: double;
                padding: 10px;
            }
            .zarban_gameboard h4 {
                border-bottom: 1px solid #00AA20; margin-top: 0px;
            }
            #${this.container.id} a {
                background-color: #333; color: #00AA20;
                font-family: "Lucida Console", "Courier New", monospace; font-size: 1em;text-decoration: none;
                display: block;
            }
            #${this.container.id} a:hover { color: #00CC40; }
            .zarban_controls a { margin-top: 10px; }
        `;
    document.head.appendChild(css);
  }
  /**
   * Save the game state to local storage
   */
  save() {
    localStorage[`zarban_${this.container.id}`] = super.save();
  }
  /**
   * Render the current game state
   */
  draw() {
    const strings = this.getInterfaceStrings();
    const gameboard = this.container.getElementsByClassName(ZARBAN_GAMEBOARD_CLASS)[0];
    const controls = this.container.getElementsByClassName(ZARBAN_CONTROLS_CLASS)[0];
    strings.description = strings.description.map((s) => s.length == 0 ? "<br/>" : s);
    gameboard.innerHTML = "";
    gameboard.appendChild(
      new InlineElement("a").setAttribute("href", "#").setStyle("float", "right").setInnerHTML("[ New Game ]").setOnClick(() => this.reset()).element()
    );
    gameboard.appendChild(
      new InlineElement("h4").setInnerHTML(strings.title).element()
    );
    strings.description.map((l) => {
      gameboard.appendChild(
        new InlineElement("p").setInnerHTML(l).element()
      );
    });
    const control_p = document.createElement("p");
    control_p.innerHTML = "What do you do?<br/>";
    for (const i in strings.options) {
      control_p.appendChild(
        new InlineElement("a").setAttribute("href", "#").setInnerHTML(`> ${strings.options[i]}`).setOnClick(() => this.step(parseInt(i) + 1)).element()
      );
    }
    controls.innerHTML = "";
    controls.appendChild(control_p);
  }
  /**
   * Advance the game state
   */
  step(option) {
    super.step(option);
    this.save();
  }
}
globalThis.play_zarban = (container_id) => new ZarbanWebRunner(container_id);
