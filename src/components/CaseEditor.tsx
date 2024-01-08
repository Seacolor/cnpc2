import { FC } from 'react';
import * as Types from '../Types';
import { format } from '../utils/StringUtils';

type Props = {
  cases: Types.TCaseCollection | null;
  caseValues: Types.UserTextCaseValue[];
  caseArgs: string[];
  onValuesChange: (values: Types.UserTextCaseValue[]) => void;
};

const CaseEditor: FC<Props> = ({
    cases,
    caseValues,
    caseArgs,
    onValuesChange,
  }) => {
    const onValueChange = (index: number, value: string) => {
      let values = [
        ...caseValues
      ]
      values[index].value = value;
      onValuesChange && onValuesChange(values);
    }
    const onNotSwitch = (index: number, value: boolean) => {
      let values = [
        ...caseValues
      ]
      values[index].not = value;
      onValuesChange && onValuesChange(values);
    }

    return (
      <>
        { caseValues.map((caseValue: Types.UserTextCaseValue, index: number) => {
          if (caseValue.value == "," || caseValue.value == "|" || caseValue.value == "/") {
            return (
                <select key={index} onChange={(e) => {
                  onValueChange(index, e.target.value)
                }} defaultValue={caseValue.value}>
                  <option value=",">かつ</option>
                  <option value="|">か</option>
                  <option value="/">または</option>
                </select>
            )
          }
          const label = (() => {
            const c = cases?.list.find((c) => c.value === caseValue.value);
            if (c) return format(c.label, ...caseArgs)
            return ""
          })();
          return (
              <select key={index} onChange={(e) => {
                onNotSwitch(index, Boolean(e.target.value))
              }} defaultValue={caseValue.not ? 1 : 0}>
                <option value={0}>{label}</option>
                <option value={1}>{label + "でなく"}</option>
              </select>
          )
        })}
      </>
    )
  };

export default CaseEditor;