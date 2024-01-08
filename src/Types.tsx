export type TCharacter = {
  author: string;
  id: string;
  name: string;
  race: string;
  class: string;
  filter: string[];
  level: number;
  relation: number,
  sex: number,
  fix_lv: number,
  rare: number,
  spawn_type: number,
  ai_calm: number,
  ai_move: number,
  ai_dist: number,
  ai_heal: number,
  ai_act0: number,
  ai_act1: number,
  ai_act2: number,
  ai_act3: number,
  ai_act4: number,
  ai_act_sub_freq: number,
  ai_act_sub0: number,
  ai_act_sub1: number,
  ai_act_sub2: number,
  ai_act_sub3: number,
  ai_act_sub4: number,
  melee_elem_id: number,
  melee_elem_power: number,
  resist: TUserResist[],
  bit_on: number[];
  transmissivity: number,
  drop_shadow_type: number,
  c_set_pos: number,
  no_food_or_drink: boolean,
  cnpc_role: number,
  race_alias: string,
  class_alias: string,
  random_name: boolean,
  chipref: number,
  colref: number,
  user_race_enabled: boolean,
  user_race: TUserRace,
  user_class_enabled: boolean,
  user_class: TUserClass,
  init_equip_enabled: boolean,
  init_equip: TInitEquip,
  txt_talk_order: boolean,
  txt: TUserText[],
  talk_enabled: boolean,
  talk: TUserTalk,
}

export type TUserResist = {
  id: number;
  value: number;
}

export type TUserRace = {
  name: string;
  id: string;
  id2: number;
  playable: boolean;
  sex: number;
  pic: number;
  pic2: number;
  dv: number;
  pv: number;
  hp: number;
  mp: number;
  str: number;
  end: number;
  dex: number;
  per: number;
  ler: number;
  wil: number;
  mag: number;
  chr: number;
  spd: number;
  melee_style: number;
  cast_style: number;
  resist: number;
  age_rnd: number;
  age: number;
  blood: number;
  breeder: number;
  height: number;
  skill: TUserSkill[];
  race_trait: TUserTrait[];
  figure: TFigure[];
  description: string;
  desc_e: string;
}

export type TUserClass = {
  name: string;
  id: string;
  playable: boolean;
  str: number;
  end: number;
  dex: number;
  per: number;
  ler: number;
  wil: number;
  mag: number;
  chr: number;
  spd: number;
  equip: number;
  skill: TUserSkill[];
  description: string;
  desc_e: string;
}

export type TUserSkill = {
  id: number;
  value: number;
}

export type TSkillListItem = {
  id: TSkill;
  value: number;
}

export type TUserTrait = {
  id: number;
  value: number;
}

export type TTraitListItem = {
  id: TTrait;
  value: TTraitValue;
}

export type TInitEquip = {
  head: string;
  head_custom_item_id: string;
  weapon1: string;
  weapon1_custom_item_id: string;
  shield: string;
  shield_custom_item_id: string;
  shoot: string;
  shoot_custom_item_id: string;
  ammo: string;
  ammo_custom_item_id: string;
  weapon2: string;
  weapon2_custom_item_id: string;
  body: string;
  body_custom_item_id: string;
  arm: string;
  arm_custom_item_id: string;
  leg: string;
  leg_custom_item_id: string;
  back: string;
  back_custom_item_id: string;
  waist: string;
  waist_custom_item_id: string;
  ring1: string;
  ring1_custom_item_id: string;
  ring2: string;
  ring2_custom_item_id: string;
  neck1: string;
  neck1_custom_item_id: string;
  neck2: string;
  neck2_custom_item_id: string;
}

export type TUserText = {
  tag: string;
  value: string;
  bodies: TUserTextBody[];
}

export type UserTextCaseValue = {
  value: string;
  not: boolean;
}

export type TUserTextBody = {
  case_values: UserTextCaseValue[];
  case_args: string[];
  jp: string[];
}

export type TRaceCollection = {
  list: [TRace];
}

export type TRace = {
  name: string;
  id: string;
  playable: boolean;
}

export type TClassCollection = {
  list: [TClass];
}

export type TClass = {
  name: string;
  id: string;
  playable: boolean;
}

export type TActionCollection = {
  list: [TAction];
}

export type TAction = {
  id: number;
  name: string;
  name_e: string;
}

export type TElementCollection = {
  list: [TElement];
}

export type TElement = {
  id: number;
  name: string;
  name_e: string;
}

export type TResistCollection = {
  list: [TResist];
}

export type TResist = {
  id: TElement;
  value: TResistValue;
}

export type TResistValueCollection = {
  list: [TResistValue];
}

export type TResistValue = {
  value: number;
  label: string;
  label_e: string;
}

export type TBitCollection = {
  list: [TBit];
}

export type TBit = {
  value: number;
  label: string;
  label_e: string;
}

export type TSkillCollection = {
  list: [TSkill];
}

export type TSkill = {
  id: number;
  name: string;
  name_e: string;
}

export type TTraitCollection = {
  list: [TTrait];
}

export type TTrait = {
  id: number;
  group: number,
  text: string;
  text_e: string,
}

export type TTraitValueCollection = {
  list: [TTraitValue];
}

export type TTraitValue = {
  id: number;
  value: number;
  text: string;
  text_e: string,
}

export type TFigureCollection = {
  list: [TFigure];
}

export type TFigure = {
  value: string;
}

export type TItemCollection = {
  list: [TItem];
}

export type TItem = {
  id: string;
  reftype: number;
  reftypeminor: number;
  name: string;
  name_e: string,
}

export type TTextCollection = {
  list: [TText];
}

export type TText = {
  tag: string;
  label: string,
}

export type TTextListItem = {
  tag: TText;
  value: string;
  bodies: [TUserTextBody];
}

export type TTextBodyListItem = {
  case_values: [UserTextCaseValue];
  case_args: [string];
  jp: [string];
}

export type TCaseGroupCollection = {
  list: [TCaseGroup];
}

export type TCaseGroup = {
  expression: string;
  label: string;
}

export type TCaseCollection = {
  list: [TCase];
}

export type TCase = {
  expression: string;
  value: string;
  args_size: number;
  args_type: string;
  label: string;
}

export type TUserTalk = {
  jp: string;
}
