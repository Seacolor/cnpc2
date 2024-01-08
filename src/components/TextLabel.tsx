import { FC } from 'react';
import * as Types from '../Types';
import { format } from '../utils/StringUtils';

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