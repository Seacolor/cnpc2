import { useState, useEffect, FC } from 'react';
import Modal from 'react-modal';
import 'bootstrap/dist/css/bootstrap.min.css';
import { invoke } from '@tauri-apps/api/tauri';
import { open, save } from '@tauri-apps/api/dialog';
import classNames from 'classnames';
import * as Types from './Types';
import "./App.css";
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';
import TextLabel from './components/TextLabel';
import ValueEditor from './components/ValueEditor'
import ArgsEditor from './components/ArgsEditor'
import CaseLabel from './components/CaseLabel'
import CaseEditor from './components/CaseEditor'
import { arrayEqual } from './utils/StringUtils';

type CaseProps = {
  id: Types.TText;
  value: string,
  bodies: [Types.TUserTextBody];
};

type BodyProps = {
  id: Types.TText;
  value: string,
  caseValues: [Types.UserTextCaseValue];
  caseArgs: [string];
  jp: [string];
};

function valuesEqual(a: Types.UserTextCaseValue[], b: Types.UserTextCaseValue[]): boolean {
  if (!Array.isArray(a))    return false;
  if (!Array.isArray(b))    return false;
  if (a.length != b.length) return false;
  for (var i = 0, n = a.length; i < n; ++i) {
    if (a[i].value !== b[i].value) return false;
    if (a[i].not !== b[i].not) return false;
  }
  return true;
}

Modal.setAppElement('#root');

function App() {
  const [ready, setReady] = useState<boolean>(false);
  const [character, setCharacter] = useState<Types.TCharacter>(
    {
      author: "",
      id: "",
      name: "",
      race: "",
      class: "",
      filter: [],
      level: 1,
      relation: -1,
      sex: -1,
      fix_lv: 0,
      rare: 1,
      spawn_type: 0,
      ai_calm: 1,
      ai_move: 50,
      ai_dist: 1,
      ai_heal: 0,
      ai_act0: 0,
      ai_act1: 0,
      ai_act2: 0,
      ai_act3: 0,
      ai_act4: 0,
      ai_act_sub_freq: 0,
      ai_act_sub0: 0,
      ai_act_sub1: 0,
      ai_act_sub2: 0,
      ai_act_sub3: 0,
      ai_act_sub4: 0,
      melee_elem_id: 0,
      melee_elem_power: 0,
      resist: [],
      bit_on: [],
      transmissivity: 0,
      drop_shadow_type: 0,
      c_set_pos: 16,
      no_food_or_drink: false,
      cnpc_role: 0,
      race_alias: "",
      class_alias: "",
      random_name: false,
      chipref: 0,
      colref: 0,
      user_race_enabled: false,
      user_race: {
        name: "",
        id: "",
        id2: 0,
        playable: false,
        sex: 0,
        pic: 0,
        pic2: 0,
        dv: 0,
        pv: 0,
        hp: 0,
        mp: 0,
        str: 0,
        end: 0,
        dex: 0,
        per: 0,
        ler: 0,
        wil: 0,
        mag: 0,
        chr: 0,
        spd: 1,
        melee_style: 0,
        cast_style: 0,
        resist: 0,
        age_rnd: 1,
        age: 1,
        blood: 0,
        breeder: 1,
        height: 1,
        skill: [],
        race_trait: [],
        figure: [],
        description: "",
        desc_e: "",
      },
      user_class_enabled: false,
      user_class: {
        name: "",
        id: "",
        playable: false,
        str: 0,
        end: 0,
        dex: 0,
        per: 0,
        ler: 0,
        wil: 0,
        mag: 0,
        chr: 0,
        spd: 1,
        equip: 0,
        skill: [],
        description: "",
        desc_e: "",
      },
      init_equip_enabled: false,
      init_equip: {
        head: "",
        head_custom_item_id: "",
        weapon1: "",
        weapon1_custom_item_id: "",
        shield: "",
        shield_custom_item_id: "",
        shoot: "",
        shoot_custom_item_id: "",
        ammo: "",
        ammo_custom_item_id: "",
        weapon2: "",
        weapon2_custom_item_id: "",
        body: "",
        body_custom_item_id: "",
        arm: "",
        arm_custom_item_id: "",
        leg: "",
        leg_custom_item_id: "",
        back: "",
        back_custom_item_id: "",
        waist: "",
        waist_custom_item_id: "",
        ring1: "",
        ring1_custom_item_id: "",
        ring2: "",
        ring2_custom_item_id: "",
        neck1: "",
        neck1_custom_item_id: "",
        neck2: "",
        neck2_custom_item_id: "",
      },
      txt_talk_order: false,
      txt: [
        {
          tag: "%txtCalm",
          value: "",
          bodies: [
            {
              case_values: [
                {
                  value: "",
                  not: false
                }
              ],
              case_args: [],
              jp: [
                "default"
              ],
            },
          ],
        },
        {
          tag: "%txtAggro",
          value: "",
          bodies: [
            {
              case_values: [
                {
                  value: "",
                  not: false
                }
              ],
              case_args: [],
              jp: [
                "default"
              ],
            },
          ],
        },
        {
          tag: "%txtDead",
          value: "",
          bodies: [
            {
              case_values: [
                {
                  value: "",
                  not: false
                }
              ],
              case_args: [],
              jp: [
                "default"
              ],
            },
          ],
        },
        {
          tag: "%txtKilled",
          value: "",
          bodies: [
            {
              case_values: [
                {
                  value: "",
                  not: false
                }
              ],
              case_args: [],
              jp: [
                "default"
              ],
            },
          ],
        },
        {
          tag: "%txtWelcome",
          value: "",
          bodies: [
            {
              case_values: [
                {
                  value: "",
                  not: false
                }
              ],
              case_args: [],
              jp: [
                "default"
              ],
            },
          ],
        },
        {
          tag: "%txtDialog",
          value: "",
          bodies: [
            {
              case_values: [
                {
                  value: "",
                  not: false
                }
              ],
              case_args: [],
              jp: [
                "default"
              ],
            },
          ],
        },
      ],
      talk_enabled: false,
      talk: {
        jp: "%txt_ucnpc_ev_b\n%txtevstart,JP\n%txt_ucnpc_ev_e",
      }
    }
  );
  const [races, setRaces] = useState<Types.TRaceCollection | null>(null);
  const [classes, setClasses] = useState<Types.TClassCollection | null>(null);
  const [actions, setActions] = useState<Types.TActionCollection | null>(null);
  const [elements, setElements] = useState<Types.TElementCollection | null>(null);
  const [resistValues, setResistValues] = useState<Types.TResistValueCollection | null>(null);
  const [resistId, setResistId] = useState<Types.TElement | null>(null);
  const [resistValue, setResistValue] = useState<Types.TResistValue | null>(null);
  const [bits, setBits] = useState<Types.TBitCollection | null>(null);
  const [skills, setSkills] = useState<Types.TSkillCollection | null>(null);
  const [userRaceSkillId, setUserRaceSkillId] = useState<Types.TSkill | null>(null);
  const [userRaceSkillValue, setUserRaceSkillValue] = useState<number>(1);
  const [traits, setTraits] = useState<Types.TTraitCollection | null>(null);
  const [traitValues, setTraitValues] = useState<Types.TTraitValueCollection | null>(null);
  const [currentUserRaceTraitValues, setCurrentUserRaceTraitValues] = useState<Types.TTraitValue[]>([]);
  const [userRaceTraitId, setUserRaceTraitId] = useState<Types.TTrait | null>(null);
  const [userRaceTraitValue, setUserRaceTraitValue] = useState<Types.TTraitValue | null>(null);
  const [figures, setFigures] = useState<Types.TFigureCollection | null>(null);
  const [figureValue, setFigureValue] = useState<Types.TFigure | null>(null);
  const [userClassSkillId, setUserClassSkillId] = useState<Types.TSkill | null>(null);
  const [userClassSkillValue, setUserClassSkillValue] = useState<number>(1);
  const [items, setItems] = useState<Types.TItemCollection | null>(null);
  const [texts, setTexts] = useState<Types.TTextCollection | null>(null);
  const [caseGroups, setCaseGroups] = useState<Types.TCaseGroupCollection | null>(null);
  const [cases, setCases] = useState<Types.TCaseCollection | null>(null);
  const [textTag, setTextTag] = useState<Types.TText | null>(null);
  const [textValue, setTextValue] = useState<string>("");
  const [textCaseGroup, setTextCaseGroup] = useState<Types.TCaseGroup | null>(null);
  const [currentCases, setCurrentCases] = useState<Types.TCase[]>([]);
  const [textCase, setTextCase] = useState<Types.TCase | null>(null);
  const [caseValues, setCaseValues] = useState<Types.UserTextCaseValue[]>([]);
  const [textCaseArgs, setTextCaseArgs] = useState<string[]>([]);
  const [caseArgs, setCaseArgs] = useState<string[]>([]);
  const [textBodyJP, setTextBodyJP] = useState<string>("");

  useEffect(() => {
    (async () => {
      const ready = await invoke<boolean>("is_ready", {})
        .catch(err => {
          console.error(err);
          return false
        });
      setReady(ready);
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const races = await invoke<Types.TRaceCollection>("get_races", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setRaces(races)
      if (races && races.list) {
        setCharacter({...character, race: races.list[0].id})
      }
    })();
  }, [ready]);

  useEffect(() => {
    (async () => {
      const classes = await invoke<Types.TClassCollection>("get_classes", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setClasses(classes)
      if (classes && classes.list) {
        setCharacter({...character, class: classes.list[0].id})
      }
    })();
  }, [ready]);

  useEffect(() => {
    (async () => {
      const actions = await invoke<Types.TActionCollection>("get_actions", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setActions(actions);
    })();
  }, [ready]);

  useEffect(() => {
    (async () => {
      const elements = await invoke<Types.TElementCollection>("get_elements", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setElements(elements);
      if (elements && elements.list) {
        setResistId(elements.list.filter((e) => e.id !== 64)[0]);
      }
    })();
  }, [ready]);

  useEffect(() => {
    (async () => {
      const resistValues = await invoke<Types.TResistValueCollection>("get_resist_values", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setResistValues(resistValues);
      if (resistValues && resistValues.list) {
        setResistValue(resistValues.list[0]);
      }
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const bits = await invoke<Types.TBitCollection>("get_bits", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setBits(bits);
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const skills = await invoke<Types.TSkillCollection>("get_skills", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setSkills(skills);
      if (skills && skills.list) {
        setUserRaceSkillId(skills.list[0]);
      }
    })();
  }, [ready]);

  useEffect(() => {
    (async () => {
      const traits = await invoke<Types.TTraitCollection>("get_traits", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setTraits(traits);
      if (traits && traits.list) {
        setUserRaceTraitId(traits.list[0]);
      }
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const traitValues = await invoke<Types.TTraitValueCollection>("get_trait_values", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setTraitValues(traitValues);
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const figures = await invoke<Types.TFigureCollection>("get_figures", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setFigures(figures);
      if (figures && figures.list) {
        setFigureValue(figures.list[0]);
      }
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const items = await invoke<Types.TItemCollection>("get_items", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setItems(items);
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const texts = await invoke<Types.TTextCollection>("get_texts", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setTexts(texts);
      if (texts && texts.list) {
        setTextTag(texts.list[0]);
      }
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const caseGroups = await invoke<Types.TCaseGroupCollection>("get_case_groups", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setCaseGroups(caseGroups);
      if (caseGroups && caseGroups.list) {
        setTextCaseGroup(caseGroups.list[0]);
      }
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const cases = await invoke<Types.TCaseCollection>("get_cases", {})
        .catch(err => {
          console.error(err);
          return null
        });
      setCases(cases);
    })();
  }, []);

  useEffect(() => {
    (async () => {
      if (traits && traits.list && traitValues && traitValues.list) {
        const t = traits.list[0];
        const tv = traitValues.list.filter((v) => v.id === t.id);
        if (!tv) throw Error("trait value not found.");
        setCurrentUserRaceTraitValues(tv);
        setUserRaceTraitValue(tv[0]);
      }
    })();
  }, [traits, traitValues]);

  useEffect(() => {
    (async () => {
      if (traits && traits.list && traitValues && traitValues.list && userRaceTraitId) {
        let values = traitValues?.list.filter((v) => v.id === userRaceTraitId.id);
        if (!values) throw Error("trait value not found.");
        setCurrentUserRaceTraitValues(values);
        setUserRaceTraitValue(values[0]);
      }
    })();
  }, [userRaceTraitId]);

  useEffect(() => {
    (async () => {
      if (caseGroups && caseGroups.list && cases && cases.list) {
        const cg = caseGroups.list[0];
        const c = cases.list.filter((c) => c.expression === cg.expression);
        if (!c) throw Error("case not found.");
        setCurrentCases(c);
        setTextCase(c[0]);
      }
    })();
  }, [caseGroups, cases]);

  useEffect(() => {
    (async () => {
      if (caseGroups && caseGroups.list && cases && cases.list && textCaseGroup) {
        let c = cases.list.filter((c) => c.expression === textCaseGroup.expression);
        if (!c) throw Error("case not found.");
        setCurrentCases(c);
        setTextCase(c[0]);
      }
    })();
  }, [textCaseGroup]);

  function selectedDirectory() {
    (async () => {
      // Open a selection dialog for directory
      const path = await open({
        directory: true,
      });
      if (path === null) {
        // user cancelled the selection
      } else {
        // user selected a single directory
        const ready = await invoke<boolean>("set_elona_dir", {path})
          .catch(err => {
            console.error(err);
            return false
          });
        setReady(ready);
      }
    })();
  }

  const BodyList: FC<BodyProps> = ({
    id,
    value,
    caseValues,
    caseArgs,
    jp,
  }) => {
    return (
      <ul>
        { jp.map((text: string, i: number) => {
          return (
            <li key={i}>
              <button className='small-button' onClick={(e) => {
                e.preventDefault();
                const t = character?.txt.find((t) => t.tag === id.tag && t.value === value);
                if (!t) return;
                const b = t.bodies.find((b) => valuesEqual(b.case_values, caseValues) && arrayEqual(b.case_args, caseArgs));
                if (!b) return;
                b.jp.splice(i, 1);
                setCharacter(
                  {
                    ...character,
                    txt: [
                      ...character?.txt,
                    ],
                  }
                );
              }}>
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-x" viewBox="0 0 16 16">
                  <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                </svg>
              </button>
              {text}
            </li>
          );
        })}
      </ul>
    );
  };

  const CaseList: FC<CaseProps> = ({
    id,
    value,
    bodies,
  }) => {
    return (
      <div>
        { bodies?.map((t: Types.TUserTextBody) => {
            return {
              case_values: t.case_values,
              case_args: t.case_args,
              jp: t.jp,
            } as Types.TTextBodyListItem;
          }).map((b: Types.TTextBodyListItem, i: number) => {
          return (
            <div key={i}>
              <label>
                <CaseLabel body={b} cases={cases} />:
                <BodyList id={id} value={value} caseValues={b.case_values} caseArgs={b.case_args} jp={b.jp} />
              </label>
            </div>
          );
        })}
      </div>
    );
  };

  return (
    <>
      <Container id='root'>
        <h1>Custom NPC Editor ver.2</h1>

        <form
          onSubmit={(e) => {
            e.preventDefault();
          }}
        >
          <Row>
            <Col>
              <button onClick={selectedDirectory}>Config</button>
              <button onClick={() => {
                (async () => {
                  // Open a selection dialog for text file
                  const path = await open({
                    filters: [{
                      name: 'Text',
                      extensions: ['txt']
                    }]
                  });
                  if (path === null) {
                    // user cancelled the selection
                  } else {
                    // user selected a single file
                    const character = await invoke<Types.TCharacter>("load_file", {path})
                      .catch(err => {
                        console.error(err);
                        return null
                      });
                    if (character) {
                      setCharacter({...character});
                    }
                  }
                })();
              }}>Load</button>
              <button onClick={() => {
                (async () => {
                  // Open a save dialog for text file
                  const path = await save({
                    filters: [{
                      name: 'Text',
                      extensions: ['txt']
                    }]
                  });
                  if (path === null) {
                    // user cancelled the selection
                  } else {
                    // user selected a single file
                    await invoke<Types.TCharacter>("save_file", { path: path, data: character })
                  }
                })();
              }}>Save</button>
            </Col>
          </Row>
          <Tabs
            defaultActiveKey="spec"
            id="tabs"
            className="mb-3"
          >
            <Tab eventKey="spec" title="Spec">
              <Row>
                <Col>
                  <label>
                    作者名:
                    <input
                      name="author-input"
                      placeholder="Input a author name..."
                      value={character?.author}
                      onChange={e => setCharacter({...character, author: e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    英語名:
                    <input
                      name="id-input"
                      placeholder="Input a english name..."
                      value={character?.id}
                      onChange={e => setCharacter({...character, id: e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    日本語名:
                    <input
                      name="name-input"
                      placeholder="Input a japanese name..."
                      value={character?.name}
                      onChange={e => setCharacter({...character, name: e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    種族:
                    <select
                      name="race-select"
                      disabled={character?.user_race_enabled}
                      value={character?.race}
                      onChange={e => setCharacter({...character, race: e.target.value})}
                    >
                      { races?.list.map((e: Types.TRace, i: number) => {
                        return (
                          <option key={i} value={e.id}>{e.name}</option>
                        );
                      })}
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    職業:
                    <select
                      name="class-select"
                      disabled={character?.user_class_enabled}
                      value={character?.class}
                      onChange={e => setCharacter({...character, class: e.target.value})}
                    >
                      { classes?.list.map((e: Types.TClass, i: number) => {
                        return (
                          <option key={i} value={e.id}>{e.name}</option>
                        );
                      })}
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    属性:
                    <select
                      name="filter-select"
                      multiple={true}
                      value={character?.filter}
                      onChange={e => {
                        const options = [...e.target.selectedOptions];
                        const values = options.map(option => option.value);
                        setCharacter({...character, filter: values})}
                      }
                    >
                      <option value="man">人</option>
                      <option value="dragon">ドラゴン</option>
                      <option value="undead">アンデッド</option>
                      <option value="slime">スライム</option>
                      <option value="fire">炎</option>
                      <option value="sf">sf</option>
                      <option value="yeek">イーク</option>
                      <option value="mino">ミノタウロス</option>
                      <option value="wild">野生</option>
                      <option value="pawn">駒</option>
                      <option value="shopguard">傭兵</option>
                      <option value="rogue">ごろつき</option>
                      <option value="cat">ネコ</option>
                      <option value="ether">エーテル</option>
                      <option value="horse">馬</option>
                      <option value="cnpc">カスタムNPC</option>
                      <option value="nogenerate">自然生成なし</option>
                      <option value="nodownload">ダウンロード対象外</option>
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    レベル:
                    <input
                      name="level-input"
                      type="number"
                      min={1}
                      max={2000}
                      value={character?.level}
                      onChange={e => setCharacter({...character, level: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    関係:
                    <select
                      name="relation-select"
                      value={character?.relation}
                      onChange={e => setCharacter({...character, relation: +e.target.value})}
                    >
                      <option value={-3}>敵対</option>
                      <option value={-2}>敵対一歩寸前</option>
                      <option value={-1}>無関心</option>
                      <option value={0}>中立</option>
                      <option value={10}>友好</option>
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    性別:
                    <select
                      name="sex-select"
                      value={character?.sex}
                      onChange={e => setCharacter({...character, sex: +e.target.value})}
                    >
                      <option value={-1}>ランダム</option>
                      <option value={0}>男</option>
                      <option value={1}>女</option>
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    生成品質:
                    <select
                      name="fix-lv-select"
                      value={character?.fix_lv}
                      onChange={e => setCharacter({...character, fix_lv: +e.target.value})}
                    >
                      <option value={0}>ランダム</option>
                      <option value={1}>粗悪</option>
                      <option value={2}>良質</option>
                      <option value={3}>高品質</option>
                      <option value={4}>奇跡</option>
                      <option value={5}>神器</option>
                      <option value={6}>特別</option>
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    生成頻度:
                    <input
                      name="rare-input"
                      type="number"
                      min={1}
                      max={500}
                      value={character?.rare}
                      onChange={e => setCharacter({...character, rare: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    生成条件:
                    <select
                      name="spawn-type-select"
                      value={character?.spawn_type}
                      onChange={e => setCharacter({...character, spawn_type: +e.target.value})}
                    >
                      <option value={0}>通常</option>
                      <option value={2}>通常(ユニーク)</option>
                      <option value={3}>特殊(ユニーク)</option>
                      <option value={4}>神々の休戦地、ルミエスト墓所</option>
                      <option value={5}>街</option>
                      <option value={6}>アクリ・テオラ</option>
                      <option value={7}>店、博物館</option>
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    待機時行動:
                    <select
                      name="ai-calm-select"
                      value={character?.ai_calm}
                      onChange={e => setCharacter({...character, ai_calm: +e.target.value})}
                    >
                      <option value={1}>放浪</option>
                      <option value={2}>鈍感</option>
                      <option value={3}>停止</option>
                      <option value={4}>随行</option>
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    移動確率:
                    <input
                      name="ai-move-input"
                      type="number"
                      min={0}
                      max={100}
                      value={character?.ai_move}
                      onChange={e => setCharacter({...character, ai_move: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    適正距離:
                    <input
                      name="ai-dist-input"
                      type="number"
                      min={1}
                      max={6}
                      value={character?.ai_dist}
                      onChange={e => setCharacter({...character, ai_dist: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    瀕死時行動:
                    <select
                      name="ai-heal-select"
                      value={character?.ai_heal}
                      onChange={e => setCharacter({...character, ai_heal: +e.target.value})}
                    >
                      { actions?.list.map((e: Types.TAction, i: number) => {
                        return (
                          <option key={i} value={e.id}>{e.name}</option>
                        );
                      })}
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    基本行動:
                    <div className="item-list">
                      <select
                        name="ai-act-0-select"
                        value={character?.ai_act0}
                        onChange={e => setCharacter({...character, ai_act0: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select
                        name="ai-act-1-select"
                        value={character?.ai_act1}
                        onChange={e => setCharacter({...character, ai_act1: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select
                        name="ai-act-2-select"
                        value={character?.ai_act2}
                        onChange={e => setCharacter({...character, ai_act2: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select
                        name="ai-act-3-select"
                        value={character?.ai_act3}
                        onChange={e => setCharacter({...character, ai_act3: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select
                        name="ai-act-4-select"
                        value={character?.ai_act4}
                        onChange={e => setCharacter({...character, ai_act4: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                    </div>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    特殊行動率:
                    <input
                      name="ai-act-sub-freq-input"
                      type="number"
                      min={0}
                      max={100}
                      value={character?.ai_act_sub_freq}
                      onChange={e => setCharacter({...character, ai_act_sub_freq: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    特殊行動:
                    <div className="item-list">
                      <select
                        name="ai-act-sub-0-select"
                        value={character?.ai_act_sub0}
                        onChange={e => setCharacter({...character, ai_act_sub0: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select
                        name="ai-act-sub-1-select"
                        value={character?.ai_act_sub1}
                        onChange={e => setCharacter({...character, ai_act_sub1: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select
                        name="ai-act-sub-2-select"
                        value={character?.ai_act_sub2}
                        onChange={e => setCharacter({...character, ai_act_sub2: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select
                        name="ai-act-sub-3-select"
                        value={character?.ai_act_sub3}
                        onChange={e => setCharacter({...character, ai_act_sub3: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select
                        name="ai-act-sub-4-select"
                        value={character?.ai_act_sub4}
                        onChange={e => setCharacter({...character, ai_act_sub4: +e.target.value})}
                      >
                        { actions?.list.map((e: Types.TAction, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                    </div>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    素手攻撃属性:
                    <select
                      name="melee-elem-select"
                      value={character?.melee_elem_id}
                      onChange={e => setCharacter({...character, melee_elem_id: +e.target.value})}
                    >
                      { elements?.list.map((e: Types.TElement, i: number) => {
                        return (
                          <option key={i} value={e.id}>{e.name}</option>
                        );
                      })}
                    </select>
                  </label>
                  <label>
                    属性強度:
                    <input
                      name="melee-elem-input"
                      type="number"
                      min={0}
                      max={500}
                      value={character?.melee_elem_power}
                      onChange={e => setCharacter({...character, melee_elem_power: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    耐性:
                    <div>
                      <select name="resist-id-select" onChange={(e) => {
                        const v = elements?.list.find((element) => element.id === +e.target.value);
                        if (!v) return;
                        setResistId(v);
                      }}>
                        { elements?.list.filter((e) => e.id !== 64).map((e: Types.TElement, i: number) => {
                          return (
                            <option key={i} value={e.id}>{e.name}</option>
                          );
                        })}
                      </select>
                      <select name="resist-value-select" onChange={(e) => {
                        const v = resistValues?.list.find((resistValue) => resistValue.value === +e.target.value);
                        if (!v) return;
                        setResistValue(v);
                      }}>
                        { resistValues?.list.map((e: Types.TResistValue, i: number) => {
                          return (
                            <option key={i} value={e.value}>{e.label}({e.value})</option>
                          );
                        })}
                      </select>
                      <button onClick={() => {
                        if (character?.resist.some((r) => r.id === resistId?.id)) return;
                        setCharacter(
                          {
                            ...character,
                            resist: [
                              ...character?.resist,
                              {
                                id: resistId?.id,
                                value: resistValue?.value,
                              } as Types.TUserResist
                            ],
                          }
                        )
                      }}>Add</button>
                    </div>
                    <div>
                      <ul>
                        { character?.resist?.map((r: Types.TUserResist) => {
                          return {
                            id: elements?.list.find((element) => element.id === r.id),
                            value: resistValues?.list.find((resistValue) => resistValue.value === r.value),
                          } as Types.TResist;
                        }).map((r: Types.TResist, i: number) => {
                          return (
                            <li key={i}>
                              <button className='small-button' onClick={() => {
                                setCharacter(
                                  {
                                    ...character,
                                    resist: character?.resist.filter((e) => e.id !== r.id.id),
                                  }
                                )
                              }}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-x" viewBox="0 0 16 16">
                                  <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                                </svg>
                              </button>
                              {r.id.name}に{r.value.label}
                            </li>
                          );
                        })}
                      </ul>
                    </div>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    補正:
                    <select
                      name="filter-select"
                      multiple={true}
                      value={character?.bit_on.map(b => `${b}`)}
                      onChange={e => {
                        const options = [...e.target.selectedOptions];
                        const values = options.map(option => +option.value);
                        setCharacter({...character, bit_on: values})}
                      }
                    >
                      { bits?.list.map((e: Types.TBit, i: number) => {
                        return (
                          <option key={i} value={e.value}>{e.label}</option>
                        );
                      })}
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    透過率:
                    <input
                      name="transmissivity-input"
                      type="number"
                      min={0}
                      max={256}
                      value={character?.transmissivity}
                      onChange={e => setCharacter({...character, transmissivity: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    影:
                    <input
                      name="drop-shadow-type-input"
                      type="number"
                      min={-1}
                      max={150}
                      value={character?.drop_shadow_type}
                      onChange={e => setCharacter({...character, drop_shadow_type: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    地面からの浮き具合:
                    <input
                      name="c-set-pos-input"
                      type="number"
                      min={16}
                      max={32}
                      value={character?.c_set_pos}
                      onChange={e => setCharacter({...character, c_set_pos: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    飲食しない:
                    <input
                      type="checkbox"
                      name="no-food-or-drink-input"
                      checked={character?.no_food_or_drink}
                      onChange={(e) => setCharacter({...character, no_food_or_drink: e.target.checked})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    吟遊詩人、清掃員、娼婦としての行動:
                    <select
                      name="cnpc-role-select"
                      value={character?.cnpc_role}
                      onChange={e => setCharacter({...character, cnpc_role: +e.target.value})}
                    >
                      <option value={0}>しない</option>
                      <option value={1}>吟遊詩人</option>
                      <option value={2}>清掃員</option>
                      <option value={3}>娼婦</option>
                    </select>
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    種族の別名:
                    <input
                      name="race-alias-input"
                      placeholder="Input a race alias..."
                      value={character?.race_alias}
                      onChange={e => setCharacter({...character, race_alias: e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    職業の別名:
                    <input
                      name="class-alias-input"
                      placeholder="Input a class alias..."
                      value={character?.class_alias}
                      onChange={e => setCharacter({...character, class_alias: e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    ランダムネーム:
                    <input
                      type="checkbox"
                      name="random-name-input"
                      checked={character?.random_name}
                      onChange={(e) => setCharacter({...character, random_name: e.target.checked})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    キャラチップ番号:
                    <input
                      name="chipref-input"
                      type="number"
                      min={0}
                      max={825}
                      value={character?.chipref}
                      onChange={e => setCharacter({...character, chipref: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    色番号:
                    <input
                      name="colref-input"
                      type="number"
                      min={0}
                      max={30}
                      value={character?.colref}
                      onChange={e => setCharacter({...character, colref: +e.target.value})}
                    />
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <label>
                    専用種族
                    <input
                      type="checkbox"
                      name="user-race-enabled-input"
                      checked={character?.user_race_enabled}
                      onChange={(e) => setCharacter({...character, user_race_enabled: e.target.checked})}
                    />
                    :
                  </label>
                </Col>
              </Row>
              <div className={classNames({hide: !character?.user_race_enabled})}>
                <Row>
                  <Col>
                    <label>
                      種族日本語名:
                      <input
                        name="user-race-name-input"
                        placeholder="Input a race name..."
                        value={character?.user_race.name}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, name: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      種族英語名:
                      <input
                        name="user-race-id-input"
                        placeholder="Input a english name..."
                        value={character?.user_race.id}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      プレイアブル:
                      <input
                        type="checkbox"
                        name="user-race-playable-input"
                        checked={character?.user_race.playable}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, playable: e.target.checked}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      性別(男性の確率):
                      <input
                        name="user-race-sex-input"
                        type="number"
                        min={0}
                        max={100}
                        value={character?.user_race.sex}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, sex: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      男性のグラフィック番号:
                      <input
                        name="user-race-pic-input"
                        type="number"
                        min={0}
                        max={824}
                        value={character?.user_race.pic}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, pic: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      女性のグラフィック番号:
                      <input
                        name="user-race-pic2-input"
                        type="number"
                        min={0}
                        max={824}
                        value={character?.user_race.pic2}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, pic2: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      DV:
                      <input
                        name="user-race-dv-input"
                        type="number"
                        min={0}
                        max={1000}
                        value={character?.user_race.dv}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, dv: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      PV:
                      <input
                        name="user-race-pv-input"
                        type="number"
                        min={0}
                        max={1000}
                        value={character?.user_race.pv}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, pv: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      HP:
                      <input
                        name="user-race-hp-input"
                        type="number"
                        min={0}
                        max={500}
                        value={character?.user_race.hp}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, hp: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      MP:
                      <input
                        name="user-race-mp-input"
                        type="number"
                        min={0}
                        max={500}
                        value={character?.user_race.mp}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, mp: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      筋力:
                      <input
                        name="user-race-str-input"
                        type="number"
                        min={0}
                        max={35}
                        value={character?.user_race.str}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, str: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      耐久:
                      <input
                        name="user-race-end-input"
                        type="number"
                        min={0}
                        max={35}
                        value={character?.user_race.end}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, end: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      器用:
                      <input
                        name="user-race-dex-input"
                        type="number"
                        min={0}
                        max={35}
                        value={character?.user_race.dex}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, dex: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      感覚:
                      <input
                        name="user-race-per-input"
                        type="number"
                        min={0}
                        max={35}
                        value={character?.user_race.per}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, per: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      習得:
                      <input
                        name="user-race-ler-input"
                        type="number"
                        min={0}
                        max={35}
                        value={character?.user_race.ler}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, ler: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      意思:
                      <input
                        name="user-race-wil-input"
                        type="number"
                        min={0}
                        max={35}
                        value={character?.user_race.wil}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, wil: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      魔力:
                      <input
                        name="user-race-mag-input"
                        type="number"
                        min={0}
                        max={35}
                        value={character?.user_race.mag}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, mag: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      魅力:
                      <input
                        name="user-race-chr-input"
                        type="number"
                        min={0}
                        max={35}
                        value={character?.user_race.chr}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, chr: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      速度:
                      <input
                        name="user-race-spd-input"
                        type="number"
                        min={1}
                        max={800}
                        value={character?.user_race.spd}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, spd: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      格闘スタイル:
                      <select
                        name="user-race-melee-style-select"
                        value={character?.user_race.melee_style}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, melee_style: +e.target.value}})}
                      >
                        <option value={0}>手(殴る)</option>
                        <option value={1}>爪</option>
                        <option value={3}>牙</option>
                        <option value={4}>眼</option>
                        <option value={5}>針</option>
                        <option value={6}>手(触る)</option>
                        <option value={7}>胞子</option>
                      </select>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      詠唱スタイル:
                      <select
                        name="user-race-cast-style-select"
                        value={character?.user_race.cast_style}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, cast_style: +e.target.value}})}
                      >
                        <option value={0}>詠唱</option>
                        <option value={1}>糸</option>
                        <option value={2}>体液</option>
                        <option value={3}>触手</option>
                        <option value={4}>眼</option>
                        <option value={5}>胞子</option>
                        <option value={6}>機械</option>
                      </select>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      耐性タイプ:
                      <select
                        name="user-race-resist-select"
                        value={character?.user_race.resist}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, resist: +e.target.value}})}
                      >
                        <option value={0}>なし</option>
                        <option value={1}>リッチ</option>
                        <option value={2}>妖精</option>
                        <option value={3}>メタル</option>
                      </select>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      年齢変動値:
                      <input
                        name="user-race-age-rnd-input"
                        type="number"
                        min={1}
                        max={300}
                        value={character?.user_race.age_rnd}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, age_rnd: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      年齢固定値:
                      <input
                        name="user-race-age-input"
                        type="number"
                        min={1}
                        max={999999}
                        value={character?.user_race.age}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, age: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      血飛沫タイプ:
                      <select
                        name="user-race-blood-select"
                        value={character?.user_race.blood}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, blood: +e.target.value}})}
                      >
                        <option value={0}>有機物</option>
                        <option value={1}>無機物</option>
                      </select>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      繁殖力:
                      <input
                        name="user-race-breeder-input"
                        type="number"
                        min={1}
                        max={1200}
                        value={character?.user_race.breeder}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, breeder: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      身長:
                      <input
                        name="user-race-height-input"
                        type="number"
                        min={1}
                        max={4500}
                        value={character?.user_race.height}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, height: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      スキル:
                      <div>
                        <select name="user-race-skill-id-select" onChange={(e) => {
                          const v = skills?.list.find((skill) => skill.id === +e.target.value);
                          if (!v) return;
                          setUserRaceSkillId(v);
                        }}>
                          { skills?.list.map((e: Types.TSkill, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                        <input
                          name="user-race-skill-value-input"
                          type="number"
                          min={1}
                          max={30}
                          value={userRaceSkillValue}
                          onChange={(e) => setUserRaceSkillValue(+e.target.value)}
                        />
                        <button onClick={() => {
                          if (character?.user_race.skill.some((r) => r.id === userRaceSkillId?.id)) return;
                          setCharacter(
                            {
                              ...character,
                              user_race: {
                                ...character?.user_race,
                                skill: [
                                  ...character?.user_race.skill,
                                  {
                                    id: userRaceSkillId?.id,
                                    value: userRaceSkillValue,
                                  } as Types.TUserSkill
                                ],
                              },
                            }
                          );
                        }}>Add</button>
                      </div>
                      <div>
                        <ul>
                          { character?.user_race.skill.map((s: Types.TUserSkill) => {
                              return {
                                id: skills?.list.find((skill) => skill.id === s.id),
                                value: s.value,
                              } as Types.TSkillListItem;
                            }).map((s: Types.TSkillListItem, i: number) => {
                            return (
                              <li key={i}>
                                <button className='small-button' onClick={() => {
                                  setCharacter(
                                    {
                                      ...character,
                                      user_race: {
                                        ...character?.user_race,
                                        skill: character?.user_race.skill.filter((r) => r.id !== s.id.id),
                                      },
                                    }
                                  )
                                }}>
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-x" viewBox="0 0 16 16">
                                    <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                                  </svg>
                                </button>
                                {s.id.name}: レベル{s.value}
                              </li>
                            );
                          })}
                        </ul>
                      </div>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      トレイト:
                      <div className="item-list">
                        <select name="user-race-trait-id-select" onChange={(e) => {
                          const id = traits?.list.find((t) => t.id === +e.target.value);
                          if (!id) return;
                          setUserRaceTraitId(id);
                        }}>
                          { traits?.list.map((e: Types.TTrait, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.text}</option>
                            );
                          })}
                        </select>
                        <select name="user-race-trait-value-select" onChange={(e) => {
                          const v = currentUserRaceTraitValues?.find((value) => value.value === +e.target.value);
                          if (!v) return;
                          setUserRaceTraitValue(v);
                        }} value={userRaceTraitValue?.value}>
                          { currentUserRaceTraitValues?.map((e: Types.TTraitValue, i: number) => {
                            return (
                              <option key={i} value={e.value}>{e.text}</option>
                            );
                          })}
                        </select>
                        <button onClick={() => {
                          if (character?.user_race.race_trait.some((r) => r.id === userRaceTraitId?.id)) return;
                          setCharacter(
                            {
                              ...character,
                              user_race: {
                                ...character?.user_race,
                                race_trait: [
                                  ...character?.user_race.race_trait,
                                  {
                                    id: userRaceTraitId?.id,
                                    value: userRaceTraitValue?.value,
                                  } as Types.TUserTrait
                                ],
                              },
                            }
                          );
                        }}>Add</button>
                      </div>
                      <div>
                        <ul>
                          { character?.user_race.race_trait.map((t: Types.TUserTrait) => {
                              return {
                                id: traits?.list.find((trait) => trait.id === t.id),
                                value: traitValues?.list.find((value) => value.id === t.id && value.value === t.value),
                              } as Types.TTraitListItem;
                            }).map((t: Types.TTraitListItem, i: number) => {
                            return (
                              <li key={i}>
                                <button className='small-button' onClick={() => {
                                  setCharacter(
                                    {
                                      ...character,
                                      user_race: {
                                        ...character?.user_race,
                                        race_trait: character?.user_race.race_trait.filter((e) => e.id !== t.id.id),
                                      },
                                    }
                                  );
                                }}>
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-x" viewBox="0 0 16 16">
                                    <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                                  </svg>
                                </button>
                                {t.value.text}
                              </li>
                            );
                          })}
                        </ul>
                      </div>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      部位:
                      <div>
                        <select name="user-race-figure-value-select" onChange={(e) => {
                          const f = figures?.list.find((f) => f.value == e.target.value);
                          if (!f) return;
                          setFigureValue(f);
                        }}>
                          { figures?.list.map((f: Types.TFigure, i: number) => {
                            return (
                              <option key={i} value={f.value}>{f.value}</option>
                            );
                          })}
                        </select>
                        <button onClick={() => {
                          if (!figureValue) return;
                          setCharacter(
                            {
                              ...character,
                              user_race: {
                                ...character?.user_race,
                                figure: [
                                  ...character?.user_race.figure,
                                  figureValue
                                ],
                              },
                            }
                          );
                        }}>Add</button>
                      </div>
                      <div>
                        <ul>
                          { character?.user_race.figure.map((f: Types.TFigure, i: number) => {
                            return (
                              <li key={i}>
                                <button className='small-button' onClick={() => {
                                  let figure = [
                                    ...character?.user_race.figure
                                  ]
                                  figure.splice(i, 1)
                                  setCharacter(
                                    {
                                      ...character,
                                      user_race: {
                                        ...character?.user_race,
                                        figure: figure,
                                      },
                                    }
                                  );
                                }}>
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-x" viewBox="0 0 16 16">
                                    <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                                  </svg>
                                </button>
                                {f.value}
                              </li>
                            );
                          })}
                        </ul>
                      </div>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      種族日本語説明文:
                      <textarea
                        name="user-race-description-textarea"
                        rows={4} cols={40}
                        placeholder="Input a japanese description..."
                        value={character?.user_race.description}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, description: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      種族英語説明文:
                      <textarea
                        name="user-race-description-e-textarea"
                        rows={4} cols={40}
                        placeholder="Input a english description..."
                        value={character?.user_race.desc_e}
                        onChange={e => setCharacter({...character, user_race: {...character?.user_race, desc_e: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
              </div>
              <Row>
                <Col>
                  <label>
                    専用職業
                    <input
                      type="checkbox"
                      name="user-class-enabled-input"
                      checked={character?.user_class_enabled}
                      onChange={(e) => setCharacter({...character, user_class_enabled: e.target.checked})}
                    />
                    :
                  </label>
                </Col>
              </Row>
              <div className={classNames({hide: !character?.user_class_enabled})}>
                <Row>
                  <Col>
                    <label>
                      職業日本語名:
                      <input
                        name="user-class-name-input"
                        placeholder="Input a japanese class name..."
                        value={character?.user_class.name}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, name: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      職業英語名:
                      <input
                        name="user-class-id-input"
                        placeholder="Input a english class name..."
                        value={character?.user_class.id}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      プレイアブル:
                      <input
                        type="checkbox"
                        name="user-class-playable-input"
                        checked={character?.user_class.playable}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, playable: e.target.checked}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      筋力:
                      <input
                        name="user-class-str-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.str}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, str: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      耐久:
                      <input
                        name="user-class-end-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.end}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, end: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      器用:
                      <input
                        name="user-class-dex-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.dex}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, dex: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      感覚:
                      <input
                        name="user-class-per-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.per}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, per: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      習得:
                      <input
                        name="user-class-ler-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.ler}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, ler: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      意思:
                      <input
                        name="user-class-wil-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.wil}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, wil: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      魔力:
                      <input
                        name="user-class-mag-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.mag}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, mag: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      魅力:
                      <input
                        name="user-class-chr-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.chr}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, chr: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      速度:
                      <input
                        name="user-class-spd-input"
                        type="number"
                        min={0}
                        max={30}
                        value={character?.user_class.spd}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, spd: +e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      装備タイプ:
                      <select
                        name="user-class-equip-select"
                        value={character?.user_class.equip}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, equip: +e.target.value}})}
                      >
                        <option value={1}>戦士系</option>
                        <option value={2}>魔法使い系</option>
                        <option value={3}>狩人系</option>
                        <option value={4}>ガンナー系</option>
                        <option value={5}>魔法戦士系</option>
                        <option value={6}>神官系</option>
                        <option value={7}>遺跡荒らし系</option>
                        <option value={8}>クレイモア系</option>
                        <option value={17}>ハイランダー系</option>
                      </select>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      スキル:
                      <div>
                        <select name="user-class-skill-id-select" onChange={(e) => {
                          const v = skills?.list.find((skill) => skill.id === +e.target.value);
                          if (!v) return;
                          setUserClassSkillId(v);
                        }}>
                          { skills?.list.map((e: Types.TSkill, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                        <input
                          name="user-class-skill-value-input"
                          type="number"
                          min={1}
                          max={10}
                          value={userClassSkillValue}
                          onChange={(e) => setUserClassSkillValue(+e.target.value)}
                        />
                        <button onClick={() => {
                          if (character?.user_class.skill.some((s) => s.id === userRaceSkillId?.id)) return;
                          setCharacter(
                            {
                              ...character,
                              user_class: {
                                ...character?.user_class,
                                skill: [
                                  ...character?.user_class.skill,
                                  {
                                    id: userClassSkillId?.id,
                                    value: userClassSkillValue,
                                  } as Types.TUserSkill
                                ],
                              },
                            }
                          );
                        }}>Add</button>
                      </div>
                      <div>
                        <ul>
                          { character?.user_class.skill.map((s: Types.TUserSkill) => {
                              return {
                                id: skills?.list.find((skill) => skill.id === s.id),
                                value: s.value,
                              } as Types.TSkillListItem;
                            }).map((s: Types.TSkillListItem, i: number) => {
                            return (
                              <li key={i}>
                                <button className='small-button' onClick={() => {
                                  setCharacter(
                                    {
                                      ...character,
                                      user_class: {
                                        ...character?.user_class,
                                        skill: character?.user_class.skill.filter((r) => r.id !== s.id.id),
                                      },
                                    }
                                  )
                                }}>
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-x" viewBox="0 0 16 16">
                                    <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                                  </svg>
                                </button>
                                {s.id.name}: レベル{s.value}
                              </li>
                            );
                          })}
                        </ul>
                      </div>
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      職業日本語説明文:
                      <textarea
                        name="user-class-description-textarea"
                        rows={4} cols={40}
                        placeholder="Input a japanese description..."
                        value={character?.user_class.description}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, description: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      職業英語説明文:
                      <textarea
                        name="user-class-description-e-textarea"
                        rows={4} cols={40}
                        placeholder="Input a english description..."
                        value={character?.user_class.desc_e}
                        onChange={e => setCharacter({...character, user_class: {...character?.user_class, desc_e: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
              </div>
              <Row>
                <Col>
                  <label>
                    初期装備
                    <input
                      type="checkbox"
                      name="init-equip-enabled-input"
                      checked={character?.init_equip_enabled}
                      onChange={(e) => setCharacter({...character, init_equip_enabled: e.target.checked})}
                    />
                    :
                  </label>
                </Col>
              </Row>
              <div className={classNames({hide: !character?.init_equip_enabled})}>
                <Row>
                  <Col>
                    <label>
                      頭:
                      <div>
                        <select
                          name="init-equip-head-select"
                          value={character?.init_equip.head}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, head: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 12000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.head != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-head-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.head_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, head_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      武器1:
                      <div>
                        <select
                          name="init-equip-weapon1-select"
                          value={character?.init_equip.weapon1}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, weapon1: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 10000 || e.reftype === 24000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.weapon1 != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-weapon1-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.weapon1_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, weapon1_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      盾:
                      <div>
                        <select
                          name="init-equip-shield-select"
                          value={character?.init_equip.shield}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, shield: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 14000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.shield != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-shield-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.shield_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, shield_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      遠隔:
                      <div>
                        <select
                          name="init-equip-shoot-select"
                          value={character?.init_equip.shoot}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, shoot: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 24000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.shoot != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-shoot-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.shoot_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, shoot_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      矢弾:
                      <div>
                        <select
                          name="init-equip-ammo-select"
                          value={character?.init_equip.ammo}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, ammo: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 25000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.ammo != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-ammo-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.ammo_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, ammo_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      武器2:
                      <div>
                        <select
                          name="init-equip-weapon2-select"
                          value={character?.init_equip.weapon2}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, weapon2: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 10000 || e.reftype === 24000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.weapon2 != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-weapon2-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.weapon2_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, weapon2_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      体:
                      <div>
                        <select
                          name="init-equip-body-select"
                          value={character?.init_equip.body}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, body: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 16000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.body != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-body-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.body_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, body_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      腕:
                      <div>
                        <select
                          name="init-equip-arm-select"
                          value={character?.init_equip.arm}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, arm: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 22000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.arm != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-arm-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.arm_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, arm_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      足:
                      <div>
                        <select
                          name="init-equip-leg-select"
                          value={character?.init_equip.leg}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, leg: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 18000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.leg != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-leg-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.leg_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, leg_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      背中:
                      <div>
                        <select
                          name="init-equip-back-select"
                          value={character?.init_equip.back}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, back: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 20000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.back != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-back-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.back_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, back_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      腰:
                      <div>
                        <select
                          name="init-equip-waist-select"
                          value={character?.init_equip.waist}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, waist: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 19000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.waist != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-waist-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.waist_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, waist_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      指1:
                      <div>
                        <select
                          name="init-equip-ring1-select"
                          value={character?.init_equip.ring1}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, ring1: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 32000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.ring1 != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-ring1-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.ring1_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, ring1_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      指2:
                      <div>
                        <select
                          name="init-equip-ring2-select"
                          value={character?.init_equip.ring2}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, ring2: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 32000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.ring2 != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-ring2-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.ring2_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, ring2_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      首1:
                      <div>
                        <select
                          name="init-equip-neck1-select"
                          value={character?.init_equip.neck1}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, neck1: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 34000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.neck1 != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-neck1-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.neck1_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, neck1_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
                <Row>
                  <Col>
                    <label>
                      首2:
                      <div>
                        <select
                          name="init-equip-neck2-select"
                          value={character?.init_equip.neck2}
                          onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, neck2: e.target.value}})}
                        >
                          { items?.list.filter((e: Types.TItem) => e.reftype === 0 || e.reftype === 34000).map((e: Types.TItem, i: number) => {
                            return (
                              <option key={i} value={e.id}>{e.name}</option>
                            );
                          })}
                        </select>
                      </div>
                    </label>
                  </Col>
                  <Col className={classNames({hide: character?.init_equip.neck2 != "743"})}>
                    <label>
                      カスタムアイテムの英語名:
                      <input
                        name="init-equip-neck2-custom-item-id"
                        placeholder="Input a custom item english name..."
                        value={character?.init_equip.neck2_custom_item_id}
                        onChange={e => setCharacter({...character, init_equip: {...character?.init_equip, neck2_custom_item_id: e.target.value}})}
                      />
                    </label>
                  </Col>
                </Row>
              </div>
            </Tab>
            <Tab eventKey="text" title="Text">
              <Row className="header">
                <Col>
                  <label>
                    台詞:
                    <div className='item-list'>
                      <select name="txt-tag-select" onChange={(e) => {
                        const t = texts?.list.find((t) => t.tag == e.target.value);
                        if (!t) return;
                        setTextTag(t);
                        setTextValue("");
                      }}>
                        { texts?.list.map((t: Types.TText, i: number) => {
                          return (
                            <option key={i} value={t.tag}>{t.label}</option>
                          );
                        })}
                      </select>
                      <ValueEditor textId={textTag} actions={actions} onValueChange={(value: string) => setTextValue(value)} />
                      <select name="txt-case-group-select" onChange={(e) => {
                        const g = caseGroups?.list.find((g) => g.expression == e.target.value);
                        if (!g) return;
                        setTextCaseGroup(g);
                        setTextCaseArgs([]);
                      }} value={textCaseGroup?.expression}>
                        { caseGroups?.list.map((t: Types.TCaseGroup, i: number) => {
                          return (
                            <option key={i} value={t.expression}>{t.label}</option>
                          );
                        })}
                      </select>
                      <select name="txt-case-select" onChange={(e) => {
                        const c = currentCases?.find((c) => c.value == e.target.value);
                        if (!c) return;
                        setTextCase(c);
                        setTextCaseArgs([]);
                      }} value={textCase?.value}>
                        { currentCases?.map((c: Types.TCase, i: number) => {
                          return (
                            <option key={i} value={c.value}>{c.label}</option>
                          );
                        })}
                      </select>
                      <ArgsEditor textCase={textCase} races={races} classes={classes} onArgsChange={(values: string[]) => setTextCaseArgs(values)} />
                      <button onClick={() => {
                        if (!textTag || !textCase || textCase.value == "") return;
                        const size = textCase.args_size;
                        if (textCaseArgs.length != size || textCaseArgs.some(v => v == "")) return;
                        if (caseValues.length == 0) {
                          setCaseValues(
                            [
                              ...caseValues,
                              {
                                value: textCase.value,
                                not: false,
                              },
                            ]
                          )
                        } else {
                          setCaseValues(
                            [
                              ...caseValues,
                              {
                                value: ",",
                                not: false,
                              },
                              {
                                value: textCase.value,
                                not: false,
                              },
                            ]
                          )
                        }
                        setCaseArgs(
                          [
                            ...caseArgs,
                            ...textCaseArgs,
                          ]
                        )
                      }}>Add case</button>
                      <div className={classNames({hide: caseValues.length == 0})}>
                        <button className='small-button' onClick={(e) => {
                          e.preventDefault();
                          setCaseValues([]);
                          setCaseArgs([]);
                        }}>
                          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-x" viewBox="0 0 16 16">
                            <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                          </svg>
                        </button>
                        <CaseEditor caseValues={caseValues} caseArgs={caseArgs} cases={cases} onValuesChange={(values: Types.UserTextCaseValue[]) => setCaseValues(values)} />
                      </div>
                      <textarea
                        name="txt-jp-textarea"
                        placeholder="Input a japanese text..."
                        value={textBodyJP}
                        onChange={e => setTextBodyJP(e.target.value)}
                      />
                      <button onClick={() => {
                        if (!textTag) return;
                        const t = character?.txt?.find((t) => t.tag == textTag.tag && t.value == textValue);
                        if (t) {
                          const b = (() => {
                            if (caseValues.length == 0) {
                              return t.bodies.find((b) => valuesEqual(b.case_values, [ { value: "", not: false } ]));
                            }
                            return t.bodies.find((b) => valuesEqual(b.case_values, caseValues) && arrayEqual(b.case_args, caseArgs));
                          })();
                          if (b) {
                            b.jp.push(textBodyJP);
                          } else {
                            t.bodies.push({
                              case_values: caseValues,
                              case_args: caseArgs,
                              jp: [textBodyJP],
                            } as Types.TUserTextBody);
                          }
                          setCharacter(
                            {
                              ...character,
                              txt: [
                                ...character?.txt,
                              ],
                            }
                          );
                        } else {
                          const values = (() => {
                            if (caseValues.length == 0) {
                              return [ { value: "", not: false } ];
                            }
                            return caseValues;
                          })();
                          setCharacter(
                            {
                              ...character,
                              txt: [
                                ...character?.txt,
                                {
                                  tag: textTag.tag,
                                  value: textValue,
                                  bodies: [
                                    {
                                      case_values: values,
                                      case_args: caseArgs,
                                      jp: [textBodyJP],
                                    } as Types.TUserTextBody
                                  ],
                                } as Types.TUserText
                              ],
                            }
                          );
                        }
                      }}>Add talk</button>
                    </div>
                  </label>
                </Col>
              </Row>
              <Row className="content item-list">
                <Col>
                  { texts && cases ? character?.txt.map((t: Types.TUserText) => {
                      return {
                        tag: texts?.list.find((text) => text.tag === t.tag),
                        value: t.value,
                        bodies: t.bodies,
                      } as Types.TTextListItem;
                    }).map((t: Types.TTextListItem, i: number) => {
                    return (
                      <div key={i}>
                        <label>
                          <TextLabel id={t.tag} value={t.value} />:
                          { t.tag.tag == "%txtDialog" ?
                          <div>
                            <label>
                              順番に話す
                              <input
                                type="checkbox"
                                name="user-txt-talk-order"
                                checked={character?.txt_talk_order}
                                onChange={(e) => setCharacter({...character, txt_talk_order: e.target.checked})}
                              />
                            </label>
                          </div>
                          : "" }
                          <CaseList id={t.tag} value={t.value} bodies={t.bodies} />
                        </label>
                      </div>
                    );
                  }) : ""}
                </Col>
              </Row>
            </Tab>
            <Tab eventKey="talk" title="Talk">
              <Row>
                <Col>
                  <label>
                    選択肢会話
                    <input
                      type="checkbox"
                      name="user-talk-enabled-input"
                      checked={character?.talk_enabled}
                      onChange={(e) => setCharacter({...character, talk_enabled: e.target.checked})}
                    />
                    :
                  </label>
                </Col>
              </Row>
              <Row>
                <Col>
                  <textarea
                    name="user-talk-jp-textarea"
                    rows={8} cols={80}
                    disabled={!character?.talk_enabled}
                    value={character?.talk.jp}
                    onChange={(e) => setCharacter({...character, talk: { ...character?.talk, jp: e.target.value }})}
                  />
                </Col>
              </Row>
            </Tab>
          </Tabs>
        </form>
      </Container>
      <Modal isOpen={!ready}>
        <h2>初回設定</h2>
        <div>始めに elona_omake_overhaul のフォルダを選択してください。</div>
        <button onClick={selectedDirectory}>Config</button>
      </Modal>
    </>
  );
}

export default App;
