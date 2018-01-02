import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { PlayerComponent } from './player.component';
import { SharedModule } from '../shared/shared.module';
import { PlayerService } from './player.service';

@NgModule({
    imports: [
        CommonModule,
        SharedModule
    ],
    declarations: [
        PlayerComponent
    ],
    exports: [
        PlayerComponent
    ],
    providers: [
        PlayerService
    ]
})
export class PlayerModule {
}
