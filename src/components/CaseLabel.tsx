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
  body: Types.TTextBodyListItem;
};

const CaseLabel: FC<Props> = ({
    body,
  }) => {
    return (
        <>
            {format(body.case.label, ...body.values)}
        </>
    )
  };

export default CaseLabel;