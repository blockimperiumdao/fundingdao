import * as React from 'react';

import { SvgIcon } from '@mui/material';
import BIGLogoIcon from '../logo.svg';

export default function LogoIcon(props) {
    return (
        <SvgIcon componet={BIGLogoIcon} {...props}>

        </SvgIcon>
    );
}