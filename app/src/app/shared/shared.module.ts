import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatToolbarModule } from '@angular/material/toolbar';
import { FlexLayoutModule } from '@angular/flex-layout';
import {
    MatButtonModule, MatCardModule, MatGridListModule, MatIconModule, MatListModule,
    MatSidenavModule
} from '@angular/material';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { HttpClientModule } from '@angular/common/http';
import { DurationPipe } from './duration.pipe';

@NgModule({
    imports: [
        CommonModule,
        BrowserAnimationsModule,
        MatToolbarModule,
        MatSidenavModule,
        MatListModule,
        MatButtonModule,
        MatGridListModule,
        MatCardModule,
        MatIconModule,
        FlexLayoutModule,
        HttpClientModule
    ],
    exports: [
        BrowserAnimationsModule,
        MatToolbarModule,
        MatSidenavModule,
        MatListModule,
        MatGridListModule,
        MatButtonModule,
        MatCardModule,
        MatIconModule,
        FlexLayoutModule,
        HttpClientModule,
        DurationPipe
    ],
    declarations: [DurationPipe]
})
export class SharedModule {
}
