import { useState, FC } from 'react';
import * as Types from '../Types';

type Props = {
  textCase: Types.TCase | null;
  races: Types.TRaceCollection | null;
  classes: Types.TClassCollection | null;
  onValuesChange: (values: string[]) => void;
};

const ValuesEditor: FC<Props> = ({
    textCase,
    races,
    classes,
    onValuesChange,
  }) => {
    const [values, setValues] = useState<string[]>(["", "", "", ""]);
  
    const onValueChange = (index: number, value: string, size: number) => {
      values[index] = value;
      setValues([
          ...values,
      ])
      const v = [] as string[];
      for (let i:number = 0; i < size; i++) {
        v.push(values[i]);
      }
      onValuesChange && onValuesChange(v);
    }

    if (textCase && textCase.values_type != "none") {
      const size = textCase.values_size || 0;
      if (size > 0) {
        if (textCase.values_type == "year") {
          return (
              <>
                <input
                  type="number"
                  min={517}
                  max={10000}
                  onChange={(e) => {
                      onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={517}
                  max={10000}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "month") {
          return (
              <>
                <input
                  type="number"
                  min={1}
                  max={12}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={1}
                  max={12}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "date") {
          return (
              <>
                <input
                  type="number"
                  min={1}
                  max={31}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={1}
                  max={31}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "hour") {
          return (
              <>
                <select onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                }}>
                  <option value="0">0</option>
                  <option value="01">1</option>
                  <option value="02">2</option>
                  <option value="03">3</option>
                  <option value="04">4</option>
                  <option value="05">5</option>
                  <option value="06">6</option>
                  <option value="07">7</option>
                  <option value="08">8</option>
                  <option value="09">9</option>
                  <option value="10">10</option>
                  <option value="11">11</option>
                  <option value="12">12</option>
                  <option value="13">13</option>
                  <option value="14">14</option>
                  <option value="15">15</option>
                  <option value="16">16</option>
                  <option value="17">17</option>
                  <option value="18">18</option>
                  <option value="19">19</option>
                  <option value="20">20</option>
                  <option value="21">21</option>
                  <option value="22">22</option>
                  <option value="23">23</option>
                </select>
                { size >= 2 ?
                <select onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                }}>
                  <option value="0">0</option>
                  <option value="01">1</option>
                  <option value="02">2</option>
                  <option value="03">3</option>
                  <option value="04">4</option>
                  <option value="05">5</option>
                  <option value="06">6</option>
                  <option value="07">7</option>
                  <option value="08">8</option>
                  <option value="09">9</option>
                  <option value="10">10</option>
                  <option value="11">11</option>
                  <option value="12">12</option>
                  <option value="13">13</option>
                  <option value="14">14</option>
                  <option value="15">15</option>
                  <option value="16">16</option>
                  <option value="17">17</option>
                  <option value="18">18</option>
                  <option value="19">19</option>
                  <option value="20">20</option>
                  <option value="21">21</option>
                  <option value="22">22</option>
                  <option value="23">23</option>
                </select>
                : ""}
              </>
          );
        } else if (textCase.values_type == "time") {
          return (
              <>
                <select onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                }}>
                  <option value="0">0</option>
                  <option value="01">1</option>
                  <option value="02">2</option>
                  <option value="03">3</option>
                  <option value="04">4</option>
                  <option value="05">5</option>
                  <option value="06">6</option>
                  <option value="07">7</option>
                  <option value="08">8</option>
                  <option value="09">9</option>
                  <option value="10">10</option>
                  <option value="11">11</option>
                  <option value="12">12</option>
                  <option value="13">13</option>
                  <option value="14">14</option>
                  <option value="15">15</option>
                  <option value="16">16</option>
                  <option value="17">17</option>
                  <option value="18">18</option>
                  <option value="19">19</option>
                  <option value="20">20</option>
                  <option value="21">21</option>
                  <option value="22">22</option>
                  <option value="23">23</option>
                </select>
                { size >= 2 ?
                <select onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                }}>
                  <option value="0">0</option>
                  <option value="01">1</option>
                  <option value="02">2</option>
                  <option value="03">3</option>
                  <option value="04">4</option>
                  <option value="05">5</option>
                  <option value="06">6</option>
                  <option value="07">7</option>
                  <option value="08">8</option>
                  <option value="09">9</option>
                  <option value="10">10</option>
                  <option value="11">11</option>
                  <option value="12">12</option>
                  <option value="13">13</option>
                  <option value="14">14</option>
                  <option value="15">15</option>
                  <option value="16">16</option>
                  <option value="17">17</option>
                  <option value="18">18</option>
                  <option value="19">19</option>
                  <option value="20">20</option>
                  <option value="21">21</option>
                  <option value="22">22</option>
                  <option value="23">23</option>
                  <option value="24">24</option>
                  <option value="25">25</option>
                  <option value="26">26</option>
                  <option value="27">27</option>
                  <option value="28">28</option>
                  <option value="29">29</option>
                  <option value="30">30</option>
                  <option value="31">31</option>
                  <option value="32">32</option>
                  <option value="33">33</option>
                  <option value="34">34</option>
                  <option value="35">35</option>
                  <option value="36">36</option>
                  <option value="37">37</option>
                  <option value="38">38</option>
                  <option value="39">39</option>
                  <option value="40">40</option>
                  <option value="41">41</option>
                  <option value="42">42</option>
                  <option value="43">43</option>
                  <option value="44">44</option>
                  <option value="45">45</option>
                  <option value="46">46</option>
                  <option value="47">47</option>
                  <option value="48">48</option>
                  <option value="49">49</option>
                  <option value="50">50</option>
                  <option value="51">51</option>
                  <option value="52">52</option>
                  <option value="53">53</option>
                  <option value="54">54</option>
                  <option value="55">55</option>
                  <option value="56">56</option>
                  <option value="57">57</option>
                  <option value="58">58</option>
                  <option value="59">59</option>
                </select>
                : ""}
                { size >= 3 ?
                <select onChange={(e) => {
                    onValueChange(2, e.target.value, size)
                }}>
                  <option value="0">0</option>
                  <option value="01">1</option>
                  <option value="02">2</option>
                  <option value="03">3</option>
                  <option value="04">4</option>
                  <option value="05">5</option>
                  <option value="06">6</option>
                  <option value="07">7</option>
                  <option value="08">8</option>
                  <option value="09">9</option>
                  <option value="10">10</option>
                  <option value="11">11</option>
                  <option value="12">12</option>
                  <option value="13">13</option>
                  <option value="14">14</option>
                  <option value="15">15</option>
                  <option value="16">16</option>
                  <option value="17">17</option>
                  <option value="18">18</option>
                  <option value="19">19</option>
                  <option value="20">20</option>
                  <option value="21">21</option>
                  <option value="22">22</option>
                  <option value="23">23</option>
                </select>
                : ""}
                { size >= 4 ?
                <select onChange={(e) => {
                    onValueChange(3, e.target.value, size)
                }}>
                  <option value="0">0</option>
                  <option value="01">1</option>
                  <option value="02">2</option>
                  <option value="03">3</option>
                  <option value="04">4</option>
                  <option value="05">5</option>
                  <option value="06">6</option>
                  <option value="07">7</option>
                  <option value="08">8</option>
                  <option value="09">9</option>
                  <option value="10">10</option>
                  <option value="11">11</option>
                  <option value="12">12</option>
                  <option value="13">13</option>
                  <option value="14">14</option>
                  <option value="15">15</option>
                  <option value="16">16</option>
                  <option value="17">17</option>
                  <option value="18">18</option>
                  <option value="19">19</option>
                  <option value="20">20</option>
                  <option value="21">21</option>
                  <option value="22">22</option>
                  <option value="23">23</option>
                  <option value="24">24</option>
                  <option value="25">25</option>
                  <option value="26">26</option>
                  <option value="27">27</option>
                  <option value="28">28</option>
                  <option value="29">29</option>
                  <option value="30">30</option>
                  <option value="31">31</option>
                  <option value="32">32</option>
                  <option value="33">33</option>
                  <option value="34">34</option>
                  <option value="35">35</option>
                  <option value="36">36</option>
                  <option value="37">37</option>
                  <option value="38">38</option>
                  <option value="39">39</option>
                  <option value="40">40</option>
                  <option value="41">41</option>
                  <option value="42">42</option>
                  <option value="43">43</option>
                  <option value="44">44</option>
                  <option value="45">45</option>
                  <option value="46">46</option>
                  <option value="47">47</option>
                  <option value="48">48</option>
                  <option value="49">49</option>
                  <option value="50">50</option>
                  <option value="51">51</option>
                  <option value="52">52</option>
                  <option value="53">53</option>
                  <option value="54">54</option>
                  <option value="55">55</option>
                  <option value="56">56</option>
                  <option value="57">57</option>
                  <option value="58">58</option>
                  <option value="59">59</option>
                </select>
                : ""}
              </>
          );
        } else if (textCase.values_type == "floor") {
          return (
              <>
                <input
                  type="number"
                  min={1}
                  max={25}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={1}
                  max={25}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "weather") {
          return (
              <>
                <select onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                }}>
                  <option value="Sun">晴れ</option>
                  <option value="Rain">雨</option>
                  <option value="Snow">雪</option>
                  <option value="Hard_rain">雷雨</option>
                  <option value="Etherwind">エーテル風</option>
                </select>
              </>
          );
        } else if (textCase.values_type == "impression") {
          return (
              <>
                <input
                  type="number"
                  min={0}
                  max={300}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={0}
                  max={300}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "sleepiness") {
          return (
              <>
                <input
                  type="number"
                  min={0}
                  max={100}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={0}
                  max={100}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "karma") {
          return (
              <>
                <input
                  type="number"
                  min={-100}
                  max={60}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={-100}
                  max={60}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "cash") {
          return (
              <>
                <input
                  type="number"
                  min={0}
                  max={2147483647}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={0}
                  max={2147483647}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "fame") {
          return (
              <>
                <input
                  type="number"
                  min={0}
                  max={2147483647}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={0}
                  max={2147483647}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        } else if (textCase.values_type == "race") {
          return (
              <>
                <select onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                }}>
                  { races?.list.map((e: Types.TRace, i: number) => {
                    return (
                      <option key={i} value={e.id}>{e.name}</option>
                    );
                  })}
                </select>
              </>
          );
        } else if (textCase.values_type == "class") {
          return (
              <>
                <select onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                }}>
                  { classes?.list.map((e: Types.TClass, i: number) => {
                    return (
                      <option key={i} value={e.id}>{e.name}</option>
                    );
                  })}
                </select>
              </>
          );
        } else if (textCase.values_type == "comparison") {
          return (
              <>
                <input
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
              </>
          );
        } else if (textCase.values_type == "percent") {
          return (
              <>
                <input
                  type="number"
                  min={1}
                  max={100}
                  onChange={(e) => {
                    onValueChange(0, e.target.value, size)
                  }}
                />
                { size >= 2 ?
                <input
                  type="number"
                  min={1}
                  max={100}
                  onChange={(e) => {
                    onValueChange(1, e.target.value, size)
                  }}
                />
                : ""}
              </>
          );
        }
      }
    }
    return (<></>);
  };

export default ValuesEditor;