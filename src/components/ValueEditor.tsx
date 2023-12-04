import { useEffect, FC } from 'react';
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

    useEffect(() => {
      if (textId?.tag == "%txtcast{0}") {
        const first = actions?.list.filter((e: Types.TAction) => e.id >= 400 && e.id < 600)[0];
        if (!first) return;
        onChange(first.id.toString());
      } else if (textId?.tag == "%txtactbefore{0}" || textId?.tag == "%txtactafter{0}") {
        const first = actions?.list.filter((e: Types.TAction) => e.id >= 600)[0];
        if (!first) return;
        onChange(first.id.toString());
      }
    }, [textId]);

    if (textId?.tag == "%txtallykilled{0}") {
      return (
          <>
            <input
              onChange={(e) => {
                onChange(e.target.value)
            }}
          />
          </>
      );
    } else if (textId?.tag == "%txtcast{0}") {
      return (
          <>
            <select onChange={(e) => {
                onChange(e.target.value)
            }}>
              { actions?.list.filter((e: Types.TAction) => e.id >= 400 && e.id < 600).map((e: Types.TAction, i: number) => {
                return (
                  <option key={i} value={e.id}>{e.name}</option>
                );
              })}
            </select>
          </>
      );
    } else if (textId?.tag == "%txtpornobook{0}") {
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
    } else if (textId?.tag == "%txtactbefore{0}" || textId?.tag == "%txtactafter{0}") {
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