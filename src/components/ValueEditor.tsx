import { FC } from 'react';
import * as Types from '../Types';

type Props = {
  textId: Types.TText | null;
  actions: Types.TActionCollection | null;
  onValueChange: (value: string) => void;
};

const ValueEditor: FC<Props> = ({
    textId,
    actions,
    onValueChange,
  }) => {
    const onChange = (value: string) => {
      onValueChange && onValueChange(value);
    }

    if (textId?.id == "%txtallykilled{}") {
      return (
          <>
            <input
              onChange={(e) => {
                onChange(e.target.value)
            }}
          />
          </>
      );
    } else if (textId?.id == "%txtcast{}") {
      return (
          <>
            <select onChange={(e) => {
                onChange(e.target.value)
            }}>
              { actions?.list.filter((e: Types.TAction) => e.id < 600).map((e: Types.TAction, i: number) => {
                return (
                  <option key={i} value={e.id}>{e.name}</option>
                );
              })}
            </select>
          </>
      );
    } else if (textId?.id == "%txtpornobook{}") {
      return (
          <>
            <input
              type="number"
              onChange={(e) => {
                onChange(e.target.value)
              }}
            />
          </>
      );
    } else if (textId?.id == "%txtactbefore{}" || textId?.id == "%txtactafter{}") {
      return (
          <>
            <select onChange={(e) => {
                onChange(e.target.value)
            }}>
              { actions?.list.filter((e: Types.TAction) => e.id >= 600).map((e: Types.TAction, i: number) => {
                return (
                  <option key={i} value={e.id}>{e.name}</option>
                );
              })}
            </select>
          </>
      );
    }
    return (<></>);
  };

export default ValueEditor;