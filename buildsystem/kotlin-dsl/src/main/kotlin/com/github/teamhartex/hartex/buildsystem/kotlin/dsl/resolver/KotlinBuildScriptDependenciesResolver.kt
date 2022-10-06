/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

package com.github.teamhartex.hartex.buildsystem.kotlin.dsl.resolver

import com.github.teamhartex.hartex.buildsystem.kotlin.dsl.concurrent.concurrentFuture
import com.github.teamhartex.hartex.buildsystem.kotlin.dsl.model.IKotlinBuildScriptModel
import java.io.File
import java.lang.Exception
import java.util.concurrent.Future as IFuture
import kotlin.script.dependencies.Environment as Environment_T
import kotlin.script.dependencies.KotlinScriptExternalDependencies as IKotlinScriptExternalDependencies
import kotlin.script.dependencies.ScriptContents as IScriptContents
import kotlin.script.dependencies.ScriptDependenciesResolver as IScriptDependenciesResolver

typealias ReportSeverity_T = IScriptDependenciesResolver.ReportSeverity
typealias Position_T = IScriptContents.Position

class KotlinBuildScriptDependenciesResolver : IScriptDependenciesResolver {
  override fun resolve(
    script: IScriptContents,
    environment: Environment_T?,
    report: (ReportSeverity_T, String, Position_T?) -> Unit,
    previousDependencies: IKotlinScriptExternalDependencies?
  ): IFuture<IKotlinScriptExternalDependencies?> = concurrentFuture {
    try {
      assembleScriptDependencies(scriptFile = script.file, environment!!, previousDependencies)
    } catch (exception: Exception) {
      previousDependencies
    }
  }

  private suspend fun assembleScriptDependencies(
    scriptFile: File?,
    environment: Environment_T,
    previousDependencies: IKotlinScriptExternalDependencies?,
  ): IKotlinScriptExternalDependencies? {
    val request = createScriptModelRequest(scriptFile, environment)
    val response = DefaultKotlinBuildScriptModelRepository.requestScriptModel(request) ?: return null

    return when {
      previousDependencies != null && previousDependencies.classpath.count() > response.getClassPath().size -> previousDependencies
      else -> {
        createBuildScriptDependencies(request, response)
      }
    }
  }

  private fun createScriptModelRequest(scriptFile: File?, environment: Environment_T): KotlinBuildScriptModelRequest =
    KotlinBuildScriptModelRequest(
      javaHome = environment.javaHome,
      projectRoot = environment.projectRoot,
      scriptFile = scriptFile
    )

  private
  fun createBuildScriptDependencies(
    request: KotlinBuildScriptModelRequest,
    response: IKotlinBuildScriptModel,
  ) =
    KotlinBuildScriptDependencies(
      response.getClassPath(),
      response.getSourcePath(),
      response.getImplicitImports(),
      request.javaHome?.path
    )
}

internal class KotlinBuildScriptDependencies(
  override val classpath: Iterable<File>,
  override val sources: Iterable<File>,
  override val imports: Iterable<String>,
  override val javaHome: String? = null,
) : IKotlinScriptExternalDependencies

private object DefaultKotlinBuildScriptModelRepository : KotlinBuildScriptModelRepository()