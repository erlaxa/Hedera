import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { MatDialog } from "@angular/material/dialog";
import { FormsModule } from '@angular/forms';

@Component({
    selector: 'create-dialog',
    templateUrl: './create-dialog.component.html',
    styleUrls: ['./create-dialog.component.css'],
    imports: [CommonModule, FormsModule]
})

export class CreateDialog {
    constructor(private dialog: MatDialog) {}

    // openDialogCreate(){
    //     this.dialog.closeAll();
    //     this.dialog.open(PhraseCreateDialog, {
    //         width: '300vw',
    //         disableClose: true,
    //     })
    // }

    phraseSaved = false;

    boxes = new Array(24);
}