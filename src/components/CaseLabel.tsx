import { FC } from 'react';
import * as Types from '../Types';
import { format } from '../utils/StringUtils';

type Props = {
  cases: Types.TCaseCollection | null;
  body: Types.TTextBodyListItem;
};

const CaseLabel: FC<Props> = ({
    cases,
    body,
  }) => {
    const label = body.case_values.map(caseValue => {
      if (caseValue.value == ",") return "かつ"
      if (caseValue.value == "|") return "か"
      if (caseValue.value == "/") return "または"
      const c = cases?.list.find((c) => c.value === caseValue.value);
      if (c) return format(c.label + (caseValue.not ? "ではなく" : ""), ...body.case_args)
      return ""
    }).join("");
    return (
        <>
            {label}
        </>
    )
  };

export default CaseLabel;