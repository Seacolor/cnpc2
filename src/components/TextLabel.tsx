import { FC } from 'react';
import * as Types from '../Types';

function format(label: string, ...values: string[]): string {
  for (const [i, arg] of values.entries()) {
    const regExp = new RegExp(`\\{${i}\\}`, 'g')
    label = label.replace(regExp, arg as string)
  }
  return label
}

type Props = {
  id: Types.TText;
  value: string;
};

const TextLabel: FC<Props> = ({
  id,
  value,
}) => {
    return (
        <>
            {format(id.label, value)}
        </>
    )
  };

export default TextLabel;