/*
 * Copyright 2020  Jaap Geurts
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick 2.2

import org.kde.plasma.configuration 2.0

ConfigModel {
    ConfigCategory {
        name: i18n("Server")
        icon: "games-config-tiles"
        source: "config_server.qml"
    }

}
